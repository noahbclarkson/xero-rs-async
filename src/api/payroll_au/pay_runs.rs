use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::pay_run;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct PayRunsResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> PayRunsResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> PayRunsListRequest<'a> {
        PayRunsListRequest::new(self.api)
    }

    pub async fn get(&self, pay_run_id: Uuid) -> Result<Vec<pay_run::PayRun>, XeroError> {
        let path = format!("/PayRuns/{pay_run_id}");
        let resp: pay_run::PayRunsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.pay_runs)
    }

    pub async fn create(
        &self,
        pay_runs: Vec<pay_run::PayRun>,
    ) -> Result<Vec<pay_run::PayRun>, XeroError> {
        let body = if pay_runs.len() == 1 {
            serde_json::to_value(pay_runs.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(pay_run::PayRunsRequest { pay_runs })?
        };
        let resp: pay_run::PayRunsResponse = self
            .api
            .client_v1
            .send_request(Method::POST, "/PayRuns", None, Some(body))
            .await?;
        Ok(resp.pay_runs)
    }
}

#[derive(Debug, Clone)]
pub struct PayRunsListRequest<'a> {
    api: &'a PayrollAuApi,
    page: Option<u32>,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> PayRunsListRequest<'a> {
    fn new(api: &'a PayrollAuApi) -> Self {
        Self {
            api,
            page: None,
            where_filter: None,
            order_by: None,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn where_filter(mut self, filter: impl Into<String>) -> Self {
        self.where_filter = Some(filter.into());
        self
    }

    pub fn order_by(mut self, order: impl Into<String>) -> Self {
        self.order_by = Some(order.into());
        self
    }

    pub async fn send(self) -> Result<Vec<pay_run::PayRun>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(filter) = self.where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = self.order_by {
            query.push(("order".to_string(), order));
        }
        let resp: pay_run::PayRunsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, "/PayRuns", Some(&query), None::<()>)
            .await?;
        Ok(resp.pay_runs)
    }
}
