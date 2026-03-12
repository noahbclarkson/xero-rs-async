use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::pay_run;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct PayRunsResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> PayRunsResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> PayRunsListRequest<'a> {
        PayRunsListRequest::new(self.api)
    }

    pub async fn get(&self, pay_run_id: Uuid) -> Result<Vec<pay_run::PayRun>, XeroError> {
        let path = format!("/payruns/{pay_run_id}");
        let resp: pay_run::PayRunsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.pay_runs.unwrap_or_default())
    }

    pub async fn create(
        &self,
        pay_runs: Vec<pay_run::PayRun>,
    ) -> Result<Vec<pay_run::PayRun>, XeroError> {
        let body = serde_json::json!({ "payRuns": pay_runs });
        let resp: pay_run::PayRunsResponse = self
            .api
            .client
            .send_request(Method::POST, "/payruns", None, Some(body))
            .await?;
        Ok(resp.pay_runs.unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct PayRunsListRequest<'a> {
    api: &'a PayrollNzApi,
    page: Option<u32>,
    status: Option<String>,
}

impl<'a> PayRunsListRequest<'a> {
    fn new(api: &'a PayrollNzApi) -> Self {
        Self {
            api,
            page: None,
            status: None,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub async fn send(self) -> Result<Vec<pay_run::PayRun>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(status) = self.status {
            query.push(("status".to_string(), status));
        }
        let resp: pay_run::PayRunsResponse = self
            .api
            .client
            .send_request(Method::GET, "/payruns", Some(&query), None::<()>)
            .await?;
        Ok(resp.pay_runs.unwrap_or_default())
    }
}
