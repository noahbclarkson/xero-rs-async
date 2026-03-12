use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::settings;
use reqwest::Method;

#[derive(Debug, Clone, Copy)]
pub struct SettingsResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> SettingsResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
        Self { api }
    }

    /// Retrieves payroll settings.
    pub async fn get(&self) -> Result<settings::Settings, XeroError> {
        let resp: settings::SettingsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, "/Settings", None, None::<()>)
            .await?;
        resp.settings.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Settings not found in response".to_string(),
        })
    }
}
