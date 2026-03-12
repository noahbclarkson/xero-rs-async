//! Time resource for the XPM Practice Manager API v3.1.

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::custom_field::CustomFieldsResponse;
use crate::models::practice_manager::time::{TimeResponse, TimesResponse};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for XPM Time entries.
#[derive(Debug, Clone, Copy)]
pub struct TimeResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> TimeResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Returns a list of time sheet entries for a specific job within a date range.
    ///
    /// `from` and `to` are in `YYYYMMDD` format.
    pub async fn by_job(
        &self,
        job_number: &str,
        from: &str,
        to: &str,
    ) -> Result<TimesResponse, XeroError> {
        let path = format!("/time.api/job/{job_number}");
        let query = vec![
            ("from".into(), from.to_string()),
            ("to".into(), to.to_string()),
        ];
        self.api
            .client
            .send_request_xml(Method::GET, &path, Some(query.as_slice()))
            .await
    }

    /// Returns a list of all time sheet entries within a date range.
    ///
    /// `from` and `to` are in `YYYYMMDD` format. The maximum range is one year.
    pub async fn list(&self, from: &str, to: &str) -> Result<TimesResponse, XeroError> {
        let query = vec![
            ("from".into(), from.to_string()),
            ("to".into(), to.to_string()),
        ];
        self.api
            .client
            .send_request_xml(Method::GET, "/time.api/list", Some(query.as_slice()))
            .await
    }

    /// Returns a list of time sheet entries for a specific staff member within a date range.
    ///
    /// `from` and `to` are in `YYYYMMDD` format.
    pub async fn by_staff(
        &self,
        uuid: Uuid,
        from: &str,
        to: &str,
    ) -> Result<TimesResponse, XeroError> {
        let path = format!("/time.api/staff/{uuid}");
        let query = vec![
            ("from".into(), from.to_string()),
            ("to".into(), to.to_string()),
        ];
        self.api
            .client
            .send_request_xml(Method::GET, &path, Some(query.as_slice()))
            .await
    }

    /// Retrieves detailed information for a specific time entry.
    pub async fn get(&self, uuid: Uuid) -> Result<TimeResponse, XeroError> {
        let path = format!("/time.api/get/{uuid}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Adds a time sheet entry to a job.
    pub async fn add(&self, xml_body: &str) -> Result<TimeResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::POST, "/time.api/add", xml_body)
            .await
    }

    /// Updates a time sheet entry on a job.
    pub async fn update(&self, xml_body: &str) -> Result<TimeResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/time.api/update", xml_body)
            .await
    }

    /// Deletes a specific time sheet entry.
    pub async fn delete(&self, uuid: Uuid) -> Result<(), XeroError> {
        let path = format!("/time.api/delete/{uuid}");
        self.api
            .client
            .send_request_xml_empty_response(Method::DELETE, &path, None)
            .await
    }

    /// Retrieves custom field data for a time entry.
    pub async fn get_custom_fields(&self, uuid: Uuid) -> Result<CustomFieldsResponse, XeroError> {
        let path = format!("/time.api/get/{uuid}/customfield");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Updates custom field data for a time entry.
    pub async fn update_custom_fields(&self, uuid: Uuid, xml_body: &str) -> Result<(), XeroError> {
        let path = format!("/time.api/update/{uuid}/customfield");
        self.api
            .client
            .send_request_xml_empty_response(Method::PUT, &path, Some(xml_body))
            .await
    }
}
