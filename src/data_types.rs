#![allow(dead_code)]

use std::str::FromStr;

use anyhow::{bail, Result};
use clap::ValueEnum;
use bc_envelope::prelude::*;
use bc_components::{ARID, Digest};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DataType {
    /// ARID: Apparently Random Identifier (ur:arid)
    Arid,

    /// Boolean (`true` or `false`)
    Bool,

    /// CBOR data in hex
    Cbor,

    /// Binary byte string in hex
    Data,

    /// Date (ISO 8601)
    Date,

    /// Cryptographic digest (ur:digest)
    Digest,

    /// Envelope (ur:envelope)
    Envelope,

    /// Known Value (number or string)
    Known,

    /// Numeric value,
    Number,

    /// UTF-8 String
    String,

    /// Uniform Resource (UR)
    Ur,

    /// URI
    Uri,

    /// UUID
    Uuid,

    /// Wrapped Envelope (ur:envelope)
    Wrapped,
}

pub fn parse_data_type_to_envelope(data_type: DataType, s: Option<&str>, ur_cbor_tag_value: Option<u64>) -> Result<Envelope> {
    if let Some(s) = s {
        match data_type {
            DataType::Arid => parse_arid(s),
            DataType::Bool => parse_boolean(s),
            DataType::Cbor => parse_cbor(s),
            DataType::Data => parse_data(s),
            DataType::Date => parse_date(s),
            DataType::Digest => parse_digest(s),
            DataType::Envelope => parse_envelope(s),
            DataType::Known => parse_known_value(s),
            DataType::Number => parse_number(s),
            DataType::String => parse_string(s),
            DataType::Ur => parse_ur(s, ur_cbor_tag_value),
            DataType::Uri => parse_uri(s),
            DataType::Uuid => parse_uuid(s),
            DataType::Wrapped => parse_wrapped_envelope(s),
        }
    } else {
        bail!("No value provided")
    }
}

/// Parse an ARID from a string.
///
/// Accepts either a hex-encoded ARID or a UR-encoded ARID.
fn parse_arid(s: &str) -> Result<Envelope> {
    if let Ok(hex) = hex::decode(s) {
        let arid = ARID::from_data_ref(hex)?;
        Ok(Envelope::new(arid))
    } else if let Ok(arid) = ARID::from_ur_string(s) {
        Ok(Envelope::new(arid))
    } else {
        bail!("Invalid ARID")
    }
}

/// Parse a boolean from a string.
///
/// Accepts either "true" or "false".
fn parse_boolean(s: &str) -> Result<Envelope> {
    let boolean = bool::from_str(s)?;
    Ok(Envelope::new(boolean))
}

/// Parse a CBOR envelope from a string.
fn parse_cbor(s: &str) -> Result<Envelope> {
    let cbor = CBOR::try_from_hex(s)?;
    Ok(Envelope::new(cbor))
}

/// Parse a bytestring from a string.
fn parse_data(s: &str) -> Result<Envelope> {
    let data = CBOR::to_byte_string(hex::decode(s)?);
    Ok(Envelope::new(data))
}

/// Parse a Date from a string.
fn parse_date(s: &str) -> Result<Envelope> {
    let date = dcbor::Date::from_string(s)?;
    Ok(Envelope::new(date))
}

/// Parse a Digest from a ur:digest string.
fn parse_digest(s: &str) -> Result<Envelope> {
    let digest = Digest::from_ur_string(s)?;
    Ok(Envelope::new(digest))
}

/// Parse an Envelope from a string.
fn parse_envelope(s: &str) -> Result<Envelope> {
    Envelope::from_ur_string(s)
}

/// Parse a KnownValue from a string.
///
/// Accepts either a integer or a string.
fn parse_known_value(s: &str) -> Result<Envelope> {
    if let Ok(number) = s.parse::<u64>() {
        Ok(Envelope::new(KnownValue::new(number)))
    } else {
        with_format_context!(|context: &FormatContext| {
            let store = context.known_values();
            if let Some(known_value) = KnownValuesStore::known_value_for_name(s, Some(store)) {
                Ok(Envelope::new(known_value))
            } else {
                bail!("Unknown known value")
            }
        })
    }
}

/// Parse a numeric value from a string.
fn parse_number(s: &str) -> Result<Envelope> {
    let number = s.parse::<f64>()?;
    Ok(Envelope::new(number))
}

/// Parse a string from a string.
fn parse_string(s: &str) -> Result<Envelope> {
    Ok(Envelope::new(s))
}

/// Parse a UR from a string.
///
/// - If the UR is a ur:envelope, acts like `type envelope`.
/// - If the UR is another type, then it attempts to look up the CBOR tag for the type and
/// encodes the envelope with the tagged CBOR content of the UR.
/// - If the UR is of an unknown type, then a tag must be used to specify the CBOR tag
/// to use.
fn parse_ur(s: &str, cbor_tag_value: Option<u64>) -> Result<Envelope> {
    let ur = UR::from_ur_string(s)?;
    if ur.ur_type_str() == "envelope" {
        let envelope = Envelope::from_ur(&ur)?;
        Ok(envelope.wrap_envelope())
    } else {
        let cbor_tag = with_format_context!(|context: &FormatContext| {
            let store = context.tags();
            if let Some(tag) = store.tag_for_name(ur.ur_type_str()) {
                Some(tag)
            } else {
                cbor_tag_value.map(Tag::with_value)
            }
        });
        if let Some(cbor_tag) = cbor_tag {
            let cbor = ur.to_cbor();
            let content_cbor = CBOR::to_tagged_value(cbor_tag, cbor);
            Ok(Envelope::new(content_cbor))
        } else {
            bail!("Unknown UR type")
        }
    }
}

/// Parse a URI from a string.
fn parse_uri(s: &str) -> Result<Envelope> {
    let uri = bc_components::URI::new(s)?;
    Ok(Envelope::new(uri))
}

/// Parse a UUID from a string.
fn parse_uuid(s: &str) -> Result<Envelope> {
    let uuid = bc_components::UUID::from_str(s)?;
    Ok(Envelope::new(uuid))
}

/// Parse a wrapped envelope from a ur:envelope string.
fn parse_wrapped_envelope(s: &str) -> Result<Envelope> {
    let envelope = Envelope::from_ur_string(s)?;
    Ok(envelope.wrap_envelope())
}
