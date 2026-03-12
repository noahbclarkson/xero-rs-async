//! Shared helpers for Payroll AU models.

use std::collections::BTreeMap;

pub type ExtraFields = BTreeMap<String, serde_json::Value>;
