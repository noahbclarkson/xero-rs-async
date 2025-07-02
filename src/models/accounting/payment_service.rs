//! Model for the PaymentService resource.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentService {
    #[serde(rename = "PaymentServiceID")]
    pub payment_service_id: Uuid,
    pub payment_service_name: String,
    pub payment_service_url: String,
    pub pay_now_text: String,
    pub payment_service_type: String,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PaymentServicesResponse {
    pub payment_services: Vec<PaymentService>,
}

// Wrapper for the request
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PaymentServicesRequest {
    pub payment_services: Vec<PaymentService>,
}
