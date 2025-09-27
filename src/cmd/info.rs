use anyhow::{Result, anyhow, bail};
use bc_ur::UR;
use clap::Args;
use dcbor::prelude::*;

/// Shared arguments for supplying provenance mark `info` payloads.
#[derive(Debug, Args, Default)]
pub struct InfoArgs {
    /// Hex-encoded dCBOR to embed in the mark's `info` field.
    #[arg(long = "info-hex", value_name = "HEX")]
    pub info_hex: Option<String>,

    /// UR containing dCBOR to embed in the mark's `info` field.
    #[arg(long = "info-ur", value_name = "UR")]
    pub info_ur: Option<String>,

    /// CBOR tag value to associate with an unknown UR type.
    #[arg(long = "info-ur-tag", value_name = "TAG")]
    pub info_ur_tag: Option<u64>,
}

impl InfoArgs {
    pub fn to_cbor(&self) -> Result<Option<CBOR>> {
        if self.info_hex.is_some() && self.info_ur.is_some() {
            bail!("specify either --info-hex or --info-ur, not both");
        }

        if self.info_ur_tag.is_some() && self.info_ur.is_none() {
            bail!("--info-ur-tag requires --info-ur");
        }

        if let Some(hex_input) = &self.info_hex {
            if self.info_ur_tag.is_some() {
                bail!("--info-ur-tag is only valid with --info-ur");
            }
            return Ok(Some(parse_hex(hex_input)?));
        }

        if let Some(ur_input) = &self.info_ur {
            return Ok(Some(parse_ur(ur_input, self.info_ur_tag)?));
        }

        if self.info_ur_tag.is_some() {
            bail!("--info-ur-tag requires a UR payload");
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

fn parse_ur(input: &str, tag_override: Option<u64>) -> Result<CBOR> {
    let ur = UR::from_ur_string(input.trim())
        .map_err(|err| anyhow!("failed to parse UR info payload: {err}"))?;
    let type_str = ur.ur_type_str().to_string();

    let registered_tag =
        with_tags!(|tags: &TagsStore| tags.tag_for_name(&type_str))
            .map(|tag| tag.value());

    if registered_tag.is_some() && tag_override.is_some() {
        bail!(
            "UR type '{}' has a known CBOR tag; --info-ur-tag must not be supplied",
            type_str
        );
    }

    let expected_tag = registered_tag.or(tag_override).ok_or_else(|| {
        anyhow!(
            "UR type '{}' is not registered; supply --info-ur-tag with the CBOR tag value",
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
