//! Jobs resource for the XPM Practice Manager API v3.1.

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::custom_field::CustomFieldsResponse;
use crate::models::practice_manager::job::{
    CreateQuoteResponse, JobResponse, JobTasksResponse, JobsResponse,
};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for XPM Jobs.
#[derive(Debug, Clone, Copy)]
pub struct JobsResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> JobsResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Returns a list of current jobs.
    ///
    /// When `detailed` is `true`, each job includes full detail (tasks,
    /// milestones, notes, etc.).
    pub async fn current(&self, detailed: bool) -> Result<JobsResponse, XeroError> {
        let query: Vec<(String, String)> = if detailed {
            vec![("detailed".into(), "true".into())]
        } else {
            vec![]
        };
        let q = if query.is_empty() {
            None
        } else {
            Some(query.as_slice())
        };
        self.api
            .client
            .send_request_xml(Method::GET, "/job.api/current", q)
            .await
    }

    /// Retrieves detailed information for a specific job by its job number.
    pub async fn get(&self, job_number: &str) -> Result<JobResponse, XeroError> {
        let path = format!("/job.api/get/{job_number}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Updates the state of a specific job.
    pub async fn update_state(&self, xml_body: &str) -> Result<JobResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/job.api/state", xml_body)
            .await
    }

    /// Returns a list of all jobs created within the given date range.
    ///
    /// `from` and `to` are in `YYYYMMDD` format. The maximum range is one year.
    /// When `detailed` is `true`, each job includes full detail.
    pub async fn list(
        &self,
        from: &str,
        to: &str,
        detailed: bool,
    ) -> Result<JobsResponse, XeroError> {
        let mut query = vec![
            ("from".into(), from.to_string()),
            ("to".into(), to.to_string()),
        ];
        if detailed {
            query.push(("detailed".into(), "true".into()));
        }
        self.api
            .client
            .send_request_xml(Method::GET, "/job.api/list", Some(query.as_slice()))
            .await
    }

    /// Returns a list of all current jobs assigned to a staff member.
    pub async fn by_staff(&self, uuid: Uuid) -> Result<JobsResponse, XeroError> {
        let path = format!("/job.api/staff/{uuid}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Returns a list of all jobs for a specific client.
    pub async fn by_client(&self, uuid: Uuid) -> Result<JobsResponse, XeroError> {
        let path = format!("/job.api/client/{uuid}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Returns a list of jobs and their tasks matching the specified criteria.
    ///
    /// - `complete`: filter by completed (`true`) or uncompleted (`false`) tasks.
    /// - `due`: return jobs with tasks due on or before this date (`YYYYMMDD`).
    /// - `start`: return jobs with tasks starting on or after this date (`YYYYMMDD`).
    pub async fn tasks(
        &self,
        complete: Option<bool>,
        due: Option<&str>,
        start: Option<&str>,
    ) -> Result<JobTasksResponse, XeroError> {
        let mut query: Vec<(String, String)> = Vec::new();
        if let Some(c) = complete {
            query.push(("complete".into(), c.to_string()));
        }
        if let Some(d) = due {
            query.push(("due".into(), d.to_string()));
        }
        if let Some(s) = start {
            query.push(("start".into(), s.to_string()));
        }
        let q = if query.is_empty() {
            None
        } else {
            Some(query.as_slice())
        };
        self.api
            .client
            .send_request_xml(Method::GET, "/job.api/tasks", q)
            .await
    }

    /// Adds a new job.
    pub async fn add(&self, xml_body: &str) -> Result<JobResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::POST, "/job.api/add", xml_body)
            .await
    }

    /// Updates an existing job.
    pub async fn update(&self, xml_body: &str) -> Result<JobResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/job.api/update", xml_body)
            .await
    }

    /// Adds a task to a job.
    pub async fn add_task(&self, xml_body: &str) -> Result<JobResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::POST, "/job.api/task", xml_body)
            .await
    }

    /// Updates a task on a job.
    pub async fn update_task(&self, xml_body: &str) -> Result<JobResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/job.api/task", xml_body)
            .await
    }

    /// Completes a task on a job.
    pub async fn complete_task(&self, uuid: Uuid) -> Result<(), XeroError> {
        let path = format!("/job.api/task/{uuid}/complete");
        self.api
            .client
            .send_request_xml_empty_response(Method::PUT, &path, None)
            .await
    }

    /// Re-opens a completed task on a job.
    pub async fn reopen_task(&self, uuid: Uuid) -> Result<(), XeroError> {
        let path = format!("/job.api/task/{uuid}/reopen");
        self.api
            .client
            .send_request_xml_empty_response(Method::PUT, &path, None)
            .await
    }

    /// Reorders the tasks on a job.
    pub async fn reorder_tasks(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(Method::PUT, "/job.api/reordertasks", Some(xml_body))
            .await
    }

    /// Adds a note to a job.
    pub async fn add_note(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/job.api/note", Some(xml_body))
            .await
    }

    /// Returns a list of documents for a job.
    pub async fn list_documents(&self, job_number: &str) -> Result<JobResponse, XeroError> {
        let path = format!("/job.api/documents/{job_number}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Adds a document to a job.
    pub async fn add_document(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/job.api/document", Some(xml_body))
            .await
    }

    /// Returns a list of costs for a job.
    pub async fn list_costs(&self, job_number: &str) -> Result<JobResponse, XeroError> {
        let path = format!("/job.api/costs/{job_number}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Adds a cost to a job.
    pub async fn add_cost(&self, xml_body: &str) -> Result<JobResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::POST, "/job.api/cost", xml_body)
            .await
    }

    /// Updates a cost on a job.
    pub async fn update_cost(&self, xml_body: &str) -> Result<JobResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/job.api/cost", xml_body)
            .await
    }

    /// Assigns staff to a job.
    pub async fn assign(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(Method::PUT, "/job.api/assign", Some(xml_body))
            .await
    }

    /// Deletes a job.
    pub async fn delete(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/job.api/delete", Some(xml_body))
            .await
    }

    /// Applies an additional template to a job.
    pub async fn apply_template(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/job.api/applytemplate", Some(xml_body))
            .await
    }

    /// Creates a quote based on a job.
    pub async fn create_quote(&self, job_number: &str) -> Result<CreateQuoteResponse, XeroError> {
        let path = format!("/job.api/createquote/{job_number}");
        self.api
            .client
            .send_request_xml::<CreateQuoteResponse>(Method::POST, &path, None)
            .await
    }

    /// Creates an estimate based on a job.
    pub async fn create_estimate(
        &self,
        job_number: &str,
    ) -> Result<CreateQuoteResponse, XeroError> {
        let path = format!("/job.api/createestimate/{job_number}");
        self.api
            .client
            .send_request_xml::<CreateQuoteResponse>(Method::POST, &path, None)
            .await
    }

    /// Retrieves custom field data for a job.
    pub async fn get_custom_fields(&self, job_id: &str) -> Result<CustomFieldsResponse, XeroError> {
        let path = format!("/job.api/get/{job_id}/customfield");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Updates custom field data for a job.
    pub async fn update_custom_fields(
        &self,
        job_id: &str,
        xml_body: &str,
    ) -> Result<(), XeroError> {
        let path = format!("/job.api/update/{job_id}/customfield");
        self.api
            .client
            .send_request_xml_empty_response(Method::PUT, &path, Some(xml_body))
            .await
    }

    /// Retrieves custom field data for a job task.
    pub async fn get_task_custom_fields(
        &self,
        uuid: Uuid,
    ) -> Result<CustomFieldsResponse, XeroError> {
        let path = format!("/job.api/task/{uuid}/customfield");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Updates custom field data for a job task.
    pub async fn update_task_custom_fields(
        &self,
        uuid: Uuid,
        xml_body: &str,
    ) -> Result<(), XeroError> {
        let path = format!("/job.api/task/{uuid}/customfield");
        self.api
            .client
            .send_request_xml_empty_response(Method::PUT, &path, Some(xml_body))
            .await
    }

    /// Retrieves custom field data for a job cost.
    pub async fn get_cost_custom_fields(
        &self,
        uuid: Uuid,
    ) -> Result<CustomFieldsResponse, XeroError> {
        let path = format!("/job.api/cost/{uuid}/customfield");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Updates custom field data for a job cost.
    pub async fn update_cost_custom_fields(
        &self,
        uuid: Uuid,
        xml_body: &str,
    ) -> Result<(), XeroError> {
        let path = format!("/job.api/cost/{uuid}/customfield");
        self.api
            .client
            .send_request_xml_empty_response(Method::PUT, &path, Some(xml_body))
            .await
    }
}
