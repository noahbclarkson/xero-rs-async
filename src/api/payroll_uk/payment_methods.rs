use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::payment_method;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct PaymentMethodsResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> PaymentMethodsResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn get(&self, employee_id: Uuid) -> Result<payment_method::PaymentMethod, XeroError> {
        let path = format!("/employees/{employee_id}/paymentMethods");
        let resp: payment_method::PaymentMethodResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.payment_method.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Payment method not found in response".to_string(),
        })
    }

    pub async fn create(
        &self,
        employee_id: Uuid,
        payment_method: payment_method::PaymentMethod,
    ) -> Result<payment_method::PaymentMethod, XeroError> {
        let path = format!("/employees/{employee_id}/paymentMethods");
        let resp: payment_method::PaymentMethodResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(payment_method))
            .await?;
        resp.payment_method.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Payment method not found in response".to_string(),
        })
    }
}
