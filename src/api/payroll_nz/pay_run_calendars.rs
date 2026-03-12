use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::pay_run_calendar;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct PayRunCalendarsResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> PayRunCalendarsResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> PayRunCalendarsListRequest<'a> {
        PayRunCalendarsListRequest::new(self.api)
    }

    pub async fn get(
        &self,
        pay_run_calendar_id: Uuid,
    ) -> Result<Vec<pay_run_calendar::PayRunCalendar>, XeroError> {
        let path = format!("/payrunCalendars/{pay_run_calendar_id}");
        let resp: pay_run_calendar::PayRunCalendarResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.pay_run_calendars.unwrap_or_default())
    }

    pub async fn create(
        &self,
        calendar: pay_run_calendar::PayRunCalendar,
    ) -> Result<Vec<pay_run_calendar::PayRunCalendar>, XeroError> {
        let body = pay_run_calendar::PayRunCalendarRequest {
            pay_run_calendars: vec![calendar],
        };
        let resp: pay_run_calendar::PayRunCalendarResponse = self
            .api
            .client
            .send_request(Method::POST, "/payrunCalendars", None, Some(body))
            .await?;
        Ok(resp.pay_run_calendars.unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct PayRunCalendarsListRequest<'a> {
    api: &'a PayrollNzApi,
    page: Option<u32>,
}

impl<'a> PayRunCalendarsListRequest<'a> {
    fn new(api: &'a PayrollNzApi) -> Self {
        Self { api, page: None }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<pay_run_calendar::PayRunCalendar>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: pay_run_calendar::PayRunCalendarResponse = self
            .api
            .client
            .send_request(Method::GET, "/payrunCalendars", Some(&query), None::<()>)
            .await?;
        Ok(resp.pay_run_calendars.unwrap_or_default())
    }
}
