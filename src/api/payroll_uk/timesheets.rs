use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::timesheet;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct TimesheetsResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> TimesheetsResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> TimesheetsListRequest<'a> {
        TimesheetsListRequest::new(self.api)
    }

    pub async fn get(&self, timesheet_id: Uuid) -> Result<timesheet::Timesheet, XeroError> {
        let path = format!("/timesheets/{timesheet_id}");
        let resp: timesheet::TimesheetResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.timesheet.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Timesheet not found in response".to_string(),
        })
    }

    pub async fn create(
        &self,
        timesheet: timesheet::Timesheet,
    ) -> Result<timesheet::Timesheet, XeroError> {
        let resp: timesheet::TimesheetResponse = self
            .api
            .client
            .send_request(Method::POST, "/timesheets", None, Some(timesheet))
            .await?;
        resp.timesheet.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Timesheet not found in response".to_string(),
        })
    }

    pub async fn update_line(
        &self,
        timesheet_id: Uuid,
        line_id: Uuid,
        line: serde_json::Value,
    ) -> Result<timesheet::Timesheet, XeroError> {
        let path = format!("/timesheets/{timesheet_id}/lines/{line_id}");
        let resp: timesheet::TimesheetResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(line))
            .await?;
        resp.timesheet.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Timesheet not found in response".to_string(),
        })
    }

    pub async fn approve(&self, timesheet_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/timesheets/{timesheet_id}/approve");
        self.api
            .client
            .send_request_empty_response(Method::POST, &path, None::<()>)
            .await
    }

    pub async fn revert_to_draft(&self, timesheet_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/timesheets/{timesheet_id}/reverttodraft");
        self.api
            .client
            .send_request_empty_response(Method::POST, &path, None::<()>)
            .await
    }

    pub async fn delete(&self, timesheet_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/timesheets/{timesheet_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }

    pub async fn delete_line(&self, timesheet_id: Uuid, line_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/timesheets/{timesheet_id}/lines/{line_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct TimesheetsListRequest<'a> {
    api: &'a PayrollUkApi,
    page: Option<u32>,
    filter: Option<String>,
    status: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    sort: Option<String>,
}

impl<'a> TimesheetsListRequest<'a> {
    fn new(api: &'a PayrollUkApi) -> Self {
        Self {
            api,
            page: None,
            filter: None,
            status: None,
            start_date: None,
            end_date: None,
            sort: None,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub async fn send(self) -> Result<Vec<timesheet::Timesheet>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(filter) = self.filter {
            query.push(("filter".to_string(), filter));
        }
        if let Some(status) = self.status {
            query.push(("status".to_string(), status));
        }
        if let Some(start_date) = self.start_date {
            query.push(("startDate".to_string(), start_date));
        }
        if let Some(end_date) = self.end_date {
            query.push(("endDate".to_string(), end_date));
        }
        if let Some(sort) = self.sort {
            query.push(("sort".to_string(), sort));
        }

        let resp: timesheet::TimesheetsResponse = self
            .api
            .client
            .send_request(Method::GET, "/timesheets", Some(&query), None::<()>)
            .await?;
        Ok(resp.timesheets.unwrap_or_default())
    }
}
