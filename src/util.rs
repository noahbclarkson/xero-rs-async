//! Provides utility functions, including custom deserializers for Xero's unique data formats.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{self, de::Visitor, Deserialize, Deserializer, Serializer};

// Helper to parse the inner Xero date string like "/Date(1750136176637+0000)/"
fn parse_xero_date(s: &str) -> Result<DateTime<Utc>, String> {
    // Trim the wrapping characters
    let s = s.trim_start_matches("/Date(").trim_end_matches(")/");

    // Find the end of the numeric timestamp part. It could be followed by a '+' or '-' for the timezone, or nothing.
    let end_of_timestamp = s.find(['+', '-']).unwrap_or(s.len());

    // Extract and parse the timestamp
    let ts_str = &s[..end_of_timestamp];
    let ts = ts_str
        .parse::<i64>()
        .map_err(|e| format!("Invalid timestamp: {}", e))?;

    DateTime::from_timestamp_millis(ts).ok_or_else(|| "Invalid timestamp value".to_string())
}

pub mod xero_date_format {
    use super::*;
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("/Date({})/", date.timestamp_millis());
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_xero_date(&s).map_err(serde::de::Error::custom)
    }
}

pub mod xero_date_format_opt {
    use super::*;
    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => xero_date_format::serialize(d, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s_opt: Option<String> = Option::deserialize(deserializer)?;
        match s_opt {
            Some(s) => parse_xero_date(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

// A flexible deserializer that can handle a field that is sometimes a string, sometimes a number.
#[allow(dead_code)]
pub fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl Visitor<'_> for StringOrNumberVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or a number")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value.to_owned()))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value.to_string()))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value.to_string()))
        }

        // Handle null values
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
    }
    // Handle missing fields by deserializing into an Option first
    use serde::de::Error as SerdeError;
    let opt = Option::<serde::de::IgnoredAny>::deserialize(deserializer);
    match opt {
        Ok(_) => Err(SerdeError::custom(
            "deserialize_string_or_number only supports string, number, or null",
        )),
        Err(_) => Ok(None),
    }
}

// Naive date formatters for optional and non-optional fields
pub mod xero_naive_date_format {
    use super::*;
    #[allow(dead_code)]
    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
    }

    #[allow(dead_code)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}

pub mod xero_naive_date_format_opt {
    use super::*;
    #[allow(dead_code)]
    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => crate::util::xero_naive_date_format::serialize(d, serializer),
            None => serializer.serialize_none(),
        }
    }

    #[allow(dead_code)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s_opt: Option<String> = Option::deserialize(deserializer)?;
        match s_opt {
            Some(s) => NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}
