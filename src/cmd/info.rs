use anyhow::{Result, anyhow, bail};
use bc_ur::UR;
use clap::Args;
use dcbor::prelude::*;

/// Shared arguments for supplying provenance mark `info` payloads.
#[derive(Debug, Args, Default)]
pub struct InfoArgs {
    /// Hex-encoded dCBOR or UR payload to embed in the mark's `info` field.
    #[arg(long = "info", value_name = "PAYLOAD")]
    pub info: Option<String>,

    /// CBOR tag value to associate with an unknown UR type.
    #[arg(long = "info-tag", value_name = "TAG")]
    pub info_tag: Option<u64>,
}

impl InfoArgs {
    pub fn to_cbor(&self) -> Result<Option<CBOR>> {
        if let Some(raw) = &self.info {
            return Ok(Some(parse_info(raw, self.info_tag)?));
        }

        if self.info_tag.is_some() {
            bail!("--info-tag requires a UR payload");
        }

        Ok(None)
    }
}

fn parse_hex(input: &str) -> Result<CBOR> {
    let trimmed = input.trim();
    let hex_str = trimmed.strip_prefix("0x").unwrap_or(trimmed);
    let bytes = hex::decode(hex_str)
        .map_err(|err| anyhow!("failed to decode hex info payload: {err}"))?;
    CBOR::try_from_data(&bytes)
        .map_err(|err| anyhow!("failed to parse info payload as dCBOR: {err}"))
}

fn parse_info(raw: &str, tag_override: Option<u64>) -> Result<CBOR> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        bail!("info payload must not be empty");
    }

    if trimmed.len() >= 3 && trimmed[..3].eq_ignore_ascii_case("ur:") {
        return parse_ur_payload(trimmed, tag_override);
    }

    match parse_hex(trimmed) {
        Ok(cbor) => {
            if tag_override.is_some() {
                bail!("--info-tag is only valid when the payload is a UR");
            }
            Ok(cbor)
        }
        Err(hex_err) => match parse_ur_payload(trimmed, tag_override) {
            Ok(cbor) => Ok(cbor),
            Err(ur_err) => Err(anyhow!(
                "failed to parse --info payload as hex ({hex_err}) or UR ({ur_err})"
            )),
        },
    }
}

fn parse_ur_payload(input: &str, tag_override: Option<u64>) -> Result<CBOR> {
    let ur = UR::from_ur_string(input.trim())
        .map_err(|err| anyhow!("failed to parse UR info payload: {err}"))?;
    let type_str = ur.ur_type_str().to_string();

    let registered_tag =
        with_tags!(|tags: &TagsStore| tags.tag_for_name(&type_str))
            .map(|tag| tag.value());

    if registered_tag.is_some() && tag_override.is_some() {
        bail!(
            "UR type '{}' has a known CBOR tag; --info-tag must not be supplied",
            type_str
        );
    }

    let expected_tag = registered_tag.or(tag_override).ok_or_else(|| {
        anyhow!(
            "UR type '{}' is not registered; supply --info-tag with the CBOR tag value",
            type_str
        )
    })?;

    let cbor = ur.cbor();
    let cbor = ensure_tag(cbor, expected_tag, &type_str)?;
    Ok(cbor)
}

fn ensure_tag(
    mut cbor: CBOR,
    expected_tag: u64,
    type_str: &str,
) -> Result<CBOR> {
    match cbor.as_case() {
        CBORCase::Tagged(tag, _) => {
            if tag.value() != expected_tag {
                bail!(
                    "UR type '{}' encodes CBOR tag {} but {} was expected",
                    type_str,
                    tag.value(),
                    expected_tag
                );
            }
            Ok(cbor)
        }
        _ => {
            let tag = Tag::with_value(expected_tag);
            let inner = cbor.clone();
            cbor = CBOR::from(CBORCase::Tagged(tag, inner));
            Ok(cbor)
        }
    }
}
