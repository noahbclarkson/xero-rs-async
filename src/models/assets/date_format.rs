//! Custom date deserialization for Xero Assets API.
//!
//! Xero Assets API returns dates as datetime strings like "2023-06-07T00:00:00"
//! but we want to store them as `NaiveDate`. This module provides custom
//! deserializers to handle this format.

use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer, Serializer};

/// Deserialize a Xero datetime string to NaiveDate.
/// Handles formats like "2023-06-07T00:00:00" by extracting just the date part.
pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_xero_date(&s).map_err(serde::de::Error::custom)
}

/// Serialize NaiveDate back to Xero's datetime format.
pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Xero expects "YYYY-MM-DDT00:00:00" format
    serializer.serialize_str(&format!("{}T00:00:00", date))
}

/// Module for Option<NaiveDate> fields.
pub mod option {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    use super::parse_xero_date;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) if !s.is_empty() => parse_xero_date(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            _ => Ok(None),
        }
    }

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => serializer.serialize_str(&format!("{}T00:00:00", d)),
            None => serializer.serialize_none(),
        }
    }
}

/// Parse a Xero datetime string to NaiveDate.
/// Handles both "YYYY-MM-DD" and "YYYY-MM-DDTHH:MM:SS" formats.
fn parse_xero_date(s: &str) -> Result<NaiveDate, String> {
    // Try to parse as datetime first (most common from Xero)
    if let Some(date_part) = s.split('T').next() {
        if let Ok(date) = NaiveDate::parse_from_str(date_part, "%Y-%m-%d") {
            return Ok(date);
        }
    }

    // Fall back to trying the whole string as a date
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse date '{}': {}", s, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_xero_datetime() {
        let result = parse_xero_date("2023-06-07T00:00:00");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            NaiveDate::from_ymd_opt(2023, 6, 7).unwrap()
        );
    }

    #[test]
    fn test_parse_xero_date_only() {
        let result = parse_xero_date("2023-06-07");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            NaiveDate::from_ymd_opt(2023, 6, 7).unwrap()
        );
    }

    #[test]
    fn test_parse_invalid_date() {
        let result = parse_xero_date("not-a-date");
        assert!(result.is_err());
    }
}
