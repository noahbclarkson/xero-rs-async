use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::organisation;
use reqwest::Method;

/// Resource accessor for Organisation.
#[derive(Debug, Clone, Copy)]
pub struct OrganisationResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> OrganisationResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Retrieves the organisation details for the tenant.
    pub async fn get(&self) -> Result<Vec<organisation::Organisation>, XeroError> {
        let resp: organisation::OrganisationsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Organisation", None, None::<()>)
            .await?;
        Ok(resp.organisations)
    }

    /// Retrieves organisation actions.
    pub async fn actions(&self) -> Result<Vec<organisation::OrganisationAction>, XeroError> {
        let resp: organisation::ActionsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Organisation/Actions", None, None::<()>)
            .await?;
        Ok(resp.actions)
    }

    /// Retrieves CIS settings for the organisation (UK only).
    pub async fn cis_settings(&self) -> Result<organisation::CISSettings, XeroError> {
        let path = "/Organisation/CISSettings";
        self.api
            .client
            .send_request(Method::GET, path, None, None::<()>)
            .await
    }
}

impl AccountingApi {
    /// Retrieves organisation details for the tenant.
    pub async fn get_organisation(&self) -> Result<Vec<organisation::Organisation>, XeroError> {
        self.organisation().get().await
    }

    /// Retrieves organisation actions.
    pub async fn get_organisation_actions(
        &self,
    ) -> Result<Vec<organisation::OrganisationAction>, XeroError> {
        self.organisation().actions().await
    }

    /// Retrieves CIS settings for the organisation (UK only).
    pub async fn get_organisation_cis_settings(
        &self,
    ) -> Result<organisation::CISSettings, XeroError> {
        self.organisation().cis_settings().await
    }
}
