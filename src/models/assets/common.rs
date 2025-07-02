//! Common enums for the Assets API.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum DepreciationMethod {
    NoDepreciation,
    StraightLine,
    DiminishingValue100,
    DiminishingValue150,
    DiminishingValue200,
    FullDepreciation,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum DepreciationCalculationMethod {
    Rate,
    Life,
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AveragingMethod {
    ActualDays,
    FullMonth,
}
