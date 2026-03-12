//! Common/shared types for the XPM Practice Manager API v3.1.

use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// YesNo – custom bool wrapper for "Yes"/"No" XML strings
// ---------------------------------------------------------------------------

/// A boolean type that (de)serializes as `"Yes"` / `"No"` strings, which is
/// the convention used by many XPM XML fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YesNo(pub bool);

impl YesNo {
    pub fn as_bool(&self) -> bool {
        self.0
    }
}

impl From<bool> for YesNo {
    fn from(b: bool) -> Self {
        YesNo(b)
    }
}

impl From<YesNo> for bool {
    fn from(yn: YesNo) -> Self {
        yn.0
    }
}

impl<'de> Deserialize<'de> for YesNo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Yes" | "yes" | "YES" | "true" | "True" | "TRUE" => Ok(YesNo(true)),
            "No" | "no" | "NO" | "false" | "False" | "FALSE" | "" => Ok(YesNo(false)),
            other => Err(serde::de::Error::custom(format!(
                "expected Yes/No or true/false, got '{other}'"
            ))),
        }
    }
}

impl Serialize for YesNo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(if self.0 { "Yes" } else { "No" })
    }
}

// ---------------------------------------------------------------------------
// Reference types – lightweight refs embedded inside other resources
// ---------------------------------------------------------------------------

/// A lightweight reference to a staff member, used when staff appear as a
/// nested element inside Client, Job, Time, etc.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Staff")]
pub struct StaffRef {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
}

/// A lightweight reference to a client.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Client")]
pub struct ClientRef {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
}

/// A lightweight reference to a contact.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Contact")]
pub struct ContactRef {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
}

// ---------------------------------------------------------------------------
// Pagination types
// ---------------------------------------------------------------------------

/// Pagination block returned in paginated list responses.
#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    #[serde(rename = "Links")]
    pub links: PaginationLinks,
}

/// Links inside a pagination block.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationLinks {
    #[serde(rename = "First")]
    pub first: Option<String>,
    #[serde(rename = "Next")]
    pub next: Option<String>,
}
