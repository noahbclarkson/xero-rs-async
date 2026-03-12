use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::timesheet;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct TimesheetsResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> TimesheetsResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> TimesheetsListRequest<'a> {
        TimesheetsListRequest::new(self.api)
    }

    pub async fn get(&self, timesheet_id: Uuid) -> Result<Vec<timesheet::Timesheet>, XeroError> {
        let path = format!("/Timesheets/{timesheet_id}");
        let resp: timesheet::TimesheetsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.timesheets)
    }

    pub async fn create_or_update(
        &self,
        timesheets: Vec<timesheet::Timesheet>,
    ) -> Result<Vec<timesheet::Timesheet>, XeroError> {
        let body = if timesheets.len() == 1 {
            serde_json::to_value(timesheets.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(timesheet::TimesheetsRequest { timesheets })?
        };
        let resp: timesheet::TimesheetsResponse = self
            .api
            .client_v1
            .send_request(Method::POST, "/Timesheets", None, Some(body))
            .await?;
        Ok(resp.timesheets)
    }
}

#[derive(Debug, Clone)]
pub struct TimesheetsListRequest<'a> {
    api: &'a PayrollAuApi,
    page: Option<u32>,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> TimesheetsListRequest<'a> {
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

    pub async fn send(self) -> Result<Vec<timesheet::Timesheet>, XeroError> {
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
        let resp: timesheet::TimesheetsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, "/Timesheets", Some(&query), None::<()>)
            .await?;
        Ok(resp.timesheets)
    }
}
