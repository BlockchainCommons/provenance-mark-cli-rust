use base64::Engine as _;
use bc_components::Seed as BcSeed;
use bc_ur::URDecodable;
use provenance_mark::{PROVENANCE_SEED_LENGTH, ProvenanceSeed};

pub fn parse_seed(input: &str) -> Result<ProvenanceSeed, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("seed string is empty".to_string());
    }

    if trimmed.starts_with("ur:") {
        return parse_seed_ur(trimmed);
    }

    if let Some(seed) = parse_seed_hex(trimmed) {
        return seed;
    }

    parse_seed_base64(trimmed)
}

fn parse_seed_ur(input: &str) -> Result<ProvenanceSeed, String> {
    let seed = BcSeed::from_ur_string(input)
        .map_err(|err| format!("failed to parse seed UR: {err}"))?;
    seed_from_exact(seed.as_bytes())
}

fn parse_seed_hex(input: &str) -> Option<Result<ProvenanceSeed, String>> {
    let source = input.strip_prefix("0x").unwrap_or(input);
    if source.is_empty() {
        return None;
    }
    if !source.len().is_multiple_of(2)
        || !source.chars().all(|c| c.is_ascii_hexdigit())
    {
        return None;
    }
    Some(
        hex::decode(source)
            .map_err(|err| format!("failed to decode hex seed: {err}"))
            .and_then(|bytes| seed_from_exact(&bytes)),
    )
}

fn parse_seed_base64(input: &str) -> Result<ProvenanceSeed, String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(input)
        .map_err(|err| format!("failed to decode base64 seed: {err}"))?;
    seed_from_exact(&bytes)
}

fn seed_from_exact(bytes: &[u8]) -> Result<ProvenanceSeed, String> {
    if bytes.len() != PROVENANCE_SEED_LENGTH {
        return Err(format!(
            "seed must be {PROVENANCE_SEED_LENGTH} bytes but found {}",
            bytes.len()
        ));
    }
    let mut block = [0u8; PROVENANCE_SEED_LENGTH];
    block.copy_from_slice(bytes);
    Ok(ProvenanceSeed::from_bytes(block))
}
