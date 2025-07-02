//! Model for the Currency resource.
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Currency {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct CurrenciesResponse {
    pub currencies: Vec<Currency>,
}

// Wrapper for the request
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct CurrenciesRequest {
    pub currencies: Vec<Currency>,
}
