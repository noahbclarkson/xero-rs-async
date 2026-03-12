//! Shared helpers for Payroll UK models.

use std::collections::BTreeMap;

pub type ExtraFields = BTreeMap<String, serde_json::Value>;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct GenericRecord {
    #[serde(flatten)]
    pub fields: ExtraFields,
}
