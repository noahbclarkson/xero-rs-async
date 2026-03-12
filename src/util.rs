//! Provides utility functions, including custom deserializers for Xero's unique data formats.

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
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
        .map_err(|e| format!("Invalid timestamp: {e}"))?;

    DateTime::from_timestamp_millis(ts).ok_or_else(|| "Invalid timestamp value".to_string())
}

pub mod xero_date_format {
    use super::{parse_xero_date, serde, DateTime, Deserialize, Deserializer, Serializer, Utc};
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
    use super::{
        parse_xero_date, serde, xero_date_format, DateTime, Deserialize, Deserializer, Serializer,
        Utc,
    };
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

/// Deserialize an optional u64 that may be represented as a JSON number or string.
pub fn deserialize_opt_u64_from_string_or_number<'de, D>(
    deserializer: D,
) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptU64Visitor;

    impl<'de> Visitor<'de> for OptU64Visitor {
        type Value = Option<u64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a u64, a string containing a u64, or null")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_opt_u64_from_string_or_number(deserializer)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if value < 0 {
                return Err(E::custom("expected non-negative integer"));
            }
            Ok(Some(value as u64))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Ok(None);
            }
            trimmed
                .parse::<u64>()
                .map(Some)
                .map_err(|e| E::custom(format!("invalid integer string: {e}")))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_str(&value)
        }
    }

    deserializer.deserialize_any(OptU64Visitor)
}

// Naive date formatters for optional and non-optional fields
pub mod xero_naive_date_format {
    use super::{serde, Deserialize, Deserializer, NaiveDate, Serializer};
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
    use super::{serde, Deserialize, Deserializer, NaiveDate, Serializer};
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

fn parse_iso_datetime(s: &str) -> Result<DateTime<Utc>, String> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }
    let naive = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f")
        .map_err(|e| format!("Invalid ISO datetime: {e}"))?;
    Ok(Utc.from_utc_datetime(&naive))
}

pub mod iso_datetime_format {
    use super::{parse_iso_datetime, serde, DateTime, Deserialize, Deserializer, Serializer, Utc};

    #[allow(dead_code)]
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.to_rfc3339())
    }

    #[allow(dead_code)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_iso_datetime(&s).map_err(serde::de::Error::custom)
    }
}

pub mod iso_datetime_format_opt {
    use super::{parse_iso_datetime, serde, DateTime, Deserialize, Deserializer, Serializer, Utc};

    #[allow(dead_code)]
    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => serializer.serialize_str(&d.to_rfc3339()),
            None => serializer.serialize_none(),
        }
    }

    #[allow(dead_code)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s_opt: Option<String> = Option::deserialize(deserializer)?;
        match s_opt {
            Some(s) => parse_iso_datetime(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}
