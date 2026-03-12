use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::settings;
use reqwest::Method;

#[derive(Debug, Clone, Copy)]
pub struct SettingsResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> SettingsResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub async fn get(&self) -> Result<settings::Settings, XeroError> {
        let resp: settings::SettingsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Settings", None, None::<()>)
            .await?;
        resp.settings.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Settings not found in response".to_string(),
        })
    }

    pub async fn update(
        &self,
        settings: settings::Settings,
    ) -> Result<settings::Settings, XeroError> {
        let body = settings::SettingsRequest { settings };
        let resp: settings::SettingsResponse = self
            .api
            .client
            .send_request(Method::PUT, "/Settings", None, Some(body))
            .await?;
        resp.settings.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Settings not found in response".to_string(),
        })
    }
}
