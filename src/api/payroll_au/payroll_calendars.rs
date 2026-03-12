use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::payroll_calendar;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct PayrollCalendarsResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> PayrollCalendarsResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> PayrollCalendarsListRequest<'a> {
        PayrollCalendarsListRequest::new(self.api)
    }

    pub async fn get(
        &self,
        payroll_calendar_id: Uuid,
    ) -> Result<Vec<payroll_calendar::PayrollCalendar>, XeroError> {
        let path = format!("/PayrollCalendars/{payroll_calendar_id}");
        let resp: payroll_calendar::PayrollCalendarsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.payroll_calendars)
    }

    pub async fn create(
        &self,
        calendar: payroll_calendar::PayrollCalendar,
    ) -> Result<Vec<payroll_calendar::PayrollCalendar>, XeroError> {
        self.create_many(vec![calendar]).await
    }

    pub async fn create_many(
        &self,
        calendars: Vec<payroll_calendar::PayrollCalendar>,
    ) -> Result<Vec<payroll_calendar::PayrollCalendar>, XeroError> {
        let body = payroll_calendar::PayrollCalendarsRequest {
            payroll_calendars: calendars,
        };
        let resp: payroll_calendar::PayrollCalendarsResponse = self
            .api
            .client_v1
            .send_request(Method::POST, "/PayrollCalendars", None, Some(body))
            .await?;
        Ok(resp.payroll_calendars)
    }
}

#[derive(Debug, Clone)]
pub struct PayrollCalendarsListRequest<'a> {
    api: &'a PayrollAuApi,
    page: Option<u32>,
    where_filter: Option<String>,
    order: Option<String>,
}

impl<'a> PayrollCalendarsListRequest<'a> {
    fn new(api: &'a PayrollAuApi) -> Self {
        Self {
            api,
            page: None,
            where_filter: None,
            order: None,
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

    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    pub async fn send(self) -> Result<Vec<payroll_calendar::PayrollCalendar>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(filter) = self.where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = self.order {
            query.push(("order".to_string(), order));
        }

        let resp: payroll_calendar::PayrollCalendarsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, "/PayrollCalendars", Some(&query), None::<()>)
            .await?;
        Ok(resp.payroll_calendars)
    }
}
