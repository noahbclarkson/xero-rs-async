use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::{branding_theme, payment_service};
use reqwest::Method;
use serde::Serialize;
use uuid::Uuid;

/// Resource accessor for Branding Themes.
#[derive(Debug, Clone, Copy)]
pub struct BrandingThemesResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> BrandingThemesResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Retrieves a list of branding themes or a single theme by ID.
    pub async fn list(
        &self,
        branding_theme_id: Option<Uuid>,
    ) -> Result<Vec<branding_theme::BrandingTheme>, XeroError> {
        let path = if let Some(id) = branding_theme_id {
            format!("/BrandingThemes/{id}")
        } else {
            "/BrandingThemes".to_string()
        };
        let resp: branding_theme::BrandingThemesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.branding_themes)
    }

    /// Retrieves payment services for a branding theme.
    pub async fn payment_services(
        &self,
        branding_theme_id: Uuid,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        let path = format!("/BrandingThemes/{branding_theme_id}/PaymentServices");
        let resp: payment_service::PaymentServicesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.payment_services)
    }

    /// Adds a payment service to a branding theme.
    pub async fn add_payment_service(
        &self,
        branding_theme_id: Uuid,
        payment_service_id: Uuid,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct RequestBody {
            payment_service_id: Uuid,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct RequestWrapper {
            #[serde(rename = "PaymentServices")]
            services: Vec<RequestBody>,
        }

        let path = format!("/BrandingThemes/{branding_theme_id}/PaymentServices");
        let body = RequestWrapper {
            services: vec![RequestBody { payment_service_id }],
        };
        let resp: payment_service::PaymentServicesResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(body))
            .await?;
        Ok(resp.payment_services)
    }
}

impl AccountingApi {
    /// Retrieves a list of branding themes.
    pub async fn get_branding_themes(
        &self,
        branding_theme_id: Option<Uuid>,
    ) -> Result<Vec<branding_theme::BrandingTheme>, XeroError> {
        self.branding_themes().list(branding_theme_id).await
    }

    /// Retrieves payment services for a branding theme.
    pub async fn get_branding_theme_payment_services(
        &self,
        branding_theme_id: Uuid,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        self.branding_themes()
            .payment_services(branding_theme_id)
            .await
    }

    /// Adds a payment service to a branding theme.
    pub async fn create_branding_theme_payment_service(
        &self,
        branding_theme_id: Uuid,
        payment_service_id: Uuid,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        self.branding_themes()
            .add_payment_service(branding_theme_id, payment_service_id)
            .await
    }
}
