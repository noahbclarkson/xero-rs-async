use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::payment_service;
use reqwest::Method;

/// Resource accessor for Payment Services.
#[derive(Debug, Clone, Copy)]
pub struct PaymentServicesResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> PaymentServicesResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Retrieves payment services.
    pub async fn list(&self) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        let resp: payment_service::PaymentServicesResponse = self
            .api
            .client
            .send_request(Method::GET, "/PaymentServices", None, None::<()>)
            .await?;
        Ok(resp.payment_services)
    }

    /// Creates a payment service.
    pub async fn create(
        &self,
        payment_service: payment_service::PaymentService,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        let resp: payment_service::PaymentServicesResponse = self
            .api
            .client
            .send_request(Method::PUT, "/PaymentServices", None, Some(payment_service))
            .await?;
        Ok(resp.payment_services)
    }
}

impl AccountingApi {
    /// Retrieves payment services.
    pub async fn get_payment_services(
        &self,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        self.payment_services().list().await
    }

    /// Creates a payment service.
    pub async fn create_payment_service(
        &self,
        payment_service: payment_service::PaymentService,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        self.payment_services().create(payment_service).await
    }
}
