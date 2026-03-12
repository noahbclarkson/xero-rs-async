//! Job models for the XPM Practice Manager API v3.1.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::{ClientRef, ContactRef, StaffRef};

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// Response for list/current endpoints returning multiple jobs.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct JobsResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Jobs")]
    pub jobs: Option<JobList>,
}

/// Inner wrapper for `<Jobs>`.
#[derive(Debug, Clone, Deserialize)]
pub struct JobList {
    #[serde(rename = "Job", default)]
    pub items: Vec<Job>,
}

/// Alias: the `/job.api/tasks` endpoint returns the same structure as a jobs
/// list (jobs with their tasks pre-filtered).
pub type JobTasksResponse = JobsResponse;

/// Response for a single job GET.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct JobResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "WebUrl")]
    pub web_url: Option<String>,
    #[serde(rename = "Job")]
    pub job: Option<Job>,
}

/// Response for `POST createquote/[job number]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct CreateQuoteResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "ID")]
    pub id: Option<String>,
}

/// Response for `POST createestimate/[job number]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct CreateEstimateResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "ID")]
    pub id: Option<String>,
}

/// Response for `GET job.api/documents/[job number]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Documents")]
pub struct JobDocumentsResponse {
    #[serde(rename = "Document", default)]
    pub items: Vec<JobDocument>,
}

/// Response for `GET job.api/costs/[job number]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Costs")]
pub struct JobCostsResponse {
    #[serde(rename = "Cost", default)]
    pub items: Vec<JobCost>,
}

// ---------------------------------------------------------------------------
// Main Job struct
// ---------------------------------------------------------------------------

/// A job in XPM.
#[derive(Debug, Clone, Deserialize)]
pub struct Job {
    /// The human-readable job number, e.g. "J000309".
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "UUID")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "ClientOrderNumber")]
    pub client_order_number: Option<String>,
    #[serde(rename = "Budget")]
    pub budget: Option<String>,
    #[serde(rename = "Type")]
    pub job_type: Option<String>,
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,
    #[serde(rename = "DueDate")]
    pub due_date: Option<String>,
    #[serde(rename = "CompletedDate")]
    pub completed_date: Option<String>,
    #[serde(rename = "Client")]
    pub client: Option<ClientRef>,
    #[serde(rename = "Contact")]
    pub contact: Option<ContactRef>,
    #[serde(rename = "Manager")]
    pub manager: Option<StaffRef>,
    #[serde(rename = "Partner")]
    pub partner: Option<StaffRef>,
    #[serde(rename = "Assigned")]
    pub assigned: Option<AssignedStaffList>,
    #[serde(rename = "Tasks")]
    pub tasks: Option<JobTaskList>,
    #[serde(rename = "Milestones")]
    pub milestones: Option<MilestoneList>,
    #[serde(rename = "Notes")]
    pub notes: Option<JobNoteList>,
}

// ---------------------------------------------------------------------------
// Nested list wrappers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct AssignedStaffList {
    #[serde(rename = "Staff", default)]
    pub items: Vec<StaffRef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobTaskList {
    #[serde(rename = "Task", default)]
    pub items: Vec<JobTask>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MilestoneList {
    #[serde(rename = "Milestone", default)]
    pub items: Vec<Milestone>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobNoteList {
    #[serde(rename = "Note", default)]
    pub items: Vec<JobNote>,
}

// ---------------------------------------------------------------------------
// Sub-structs
// ---------------------------------------------------------------------------

/// A task on a job.
#[derive(Debug, Clone, Deserialize)]
pub struct JobTask {
    #[serde(rename = "UUID")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "TaskUUID")]
    pub task_uuid: Option<Uuid>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "EstimatedMinutes")]
    pub estimated_minutes: Option<String>,
    #[serde(rename = "ActualMinutes")]
    pub actual_minutes: Option<String>,
    #[serde(rename = "Completed")]
    pub completed: Option<String>,
    #[serde(rename = "Billable")]
    pub billable: Option<String>,
    #[serde(rename = "Folder")]
    pub folder: Option<String>,
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,
    #[serde(rename = "DueDate")]
    pub due_date: Option<String>,
    #[serde(rename = "Assigned")]
    pub assigned: Option<JobTaskAssignedList>,
}

/// Staff assigned to a specific job task, with optional allocated minutes.
#[derive(Debug, Clone, Deserialize)]
pub struct JobTaskAssignedStaff {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "AllocatedMinutes")]
    pub allocated_minutes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobTaskAssignedList {
    #[serde(rename = "Staff", default)]
    pub items: Vec<JobTaskAssignedStaff>,
}

/// A milestone on a job.
#[derive(Debug, Clone, Deserialize)]
pub struct Milestone {
    #[serde(rename = "UUID")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Completed")]
    pub completed: Option<String>,
    #[serde(rename = "Folder")]
    pub folder: Option<String>,
}

/// A note on a job.
#[derive(Debug, Clone, Deserialize)]
pub struct JobNote {
    #[serde(rename = "UUID")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Text")]
    pub text: Option<String>,
    #[serde(rename = "Folder")]
    pub folder: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<String>,
    #[serde(rename = "Comments")]
    pub comments: Option<JobCommentList>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobCommentList {
    #[serde(rename = "Comment", default)]
    pub items: Vec<JobComment>,
}

/// A comment on a job note.
#[derive(Debug, Clone, Deserialize)]
pub struct JobComment {
    #[serde(rename = "UUID")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "Text")]
    pub text: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<String>,
}

/// A cost entry on a job.
#[derive(Debug, Clone, Deserialize)]
pub struct JobCost {
    #[serde(rename = "UUID")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Note")]
    pub note: Option<String>,
    #[serde(rename = "Quantity")]
    pub quantity: Option<String>,
    #[serde(rename = "UnitCost")]
    pub unit_cost: Option<String>,
    #[serde(rename = "UnitPrice")]
    pub unit_price: Option<String>,
    #[serde(rename = "Billable")]
    pub billable: Option<String>,
}

/// A document linked to a job.
#[derive(Debug, Clone, Deserialize)]
pub struct JobDocument {
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Text")]
    pub text: Option<String>,
    #[serde(rename = "Folder")]
    pub folder: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<String>,
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
}

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// Request body for `POST job.api/add`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Job")]
pub struct AddJobRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ClientUUID")]
    pub client_uuid: Uuid,
    #[serde(rename = "ContactUUID", skip_serializing_if = "Option::is_none")]
    pub contact_uuid: Option<Uuid>,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "DueDate")]
    pub due_date: String,
    #[serde(rename = "ClientNumber", skip_serializing_if = "Option::is_none")]
    pub client_number: Option<String>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "TemplateUUID", skip_serializing_if = "Option::is_none")]
    pub template_uuid: Option<Uuid>,
    #[serde(rename = "CategoryUUID", skip_serializing_if = "Option::is_none")]
    pub category_uuid: Option<Uuid>,
    #[serde(rename = "Budget", skip_serializing_if = "Option::is_none")]
    pub budget: Option<String>,
}

/// Request body for `PUT job.api/update`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Job")]
pub struct UpdateJobRequest {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "ClientNumber", skip_serializing_if = "Option::is_none")]
    pub client_number: Option<String>,
    #[serde(rename = "CategoryID", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(rename = "Budget", skip_serializing_if = "Option::is_none")]
    pub budget: Option<String>,
}

/// Request body for `PUT job.api/state`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Job")]
pub struct UpdateJobStateRequest {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "State")]
    pub state: String,
}

/// Request body for `POST job.api/delete`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Job")]
pub struct DeleteJobRequest {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

/// Request body for `POST job.api/task` — add a task to a job.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Task")]
pub struct AddJobTaskRequest {
    /// The job number, e.g. "J000309".
    #[serde(rename = "Job")]
    pub job: String,
    #[serde(rename = "TaskUUID")]
    pub task_uuid: Uuid,
    #[serde(rename = "Label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EstimatedMinutes")]
    pub estimated_minutes: i64,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

/// Request body for `PUT job.api/task` — update a task on a job.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Task")]
pub struct UpdateJobTaskRequest {
    #[serde(rename = "TaskUUID")]
    pub task_uuid: Uuid,
    #[serde(rename = "Label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EstimatedMinutes", skip_serializing_if = "Option::is_none")]
    pub estimated_minutes: Option<i64>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

/// Task UUID entry for reorder.
#[derive(Debug, Clone, Serialize)]
pub struct ReorderTaskEntry {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

/// Task list for reorder.
#[derive(Debug, Clone, Serialize)]
pub struct ReorderTasksList {
    #[serde(rename = "Task")]
    pub items: Vec<ReorderTaskEntry>,
}

/// Request body for `PUT job.api/reordertasks`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Job")]
pub struct ReorderTasksRequest {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Tasks")]
    pub tasks: ReorderTasksList,
}

/// Request body for `POST job.api/note`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Note")]
pub struct AddJobNoteRequest {
    /// The job number, e.g. "J000309".
    #[serde(rename = "Job")]
    pub job: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Folder", skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(rename = "Public", skip_serializing_if = "Option::is_none")]
    pub public: Option<String>,
}

/// Request body for `POST job.api/document`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Document")]
pub struct AddJobDocumentRequest {
    /// The job number, e.g. "J000309".
    #[serde(rename = "Job")]
    pub job: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Folder", skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(rename = "Public", skip_serializing_if = "Option::is_none")]
    pub public: Option<String>,
    #[serde(rename = "FileName")]
    pub file_name: String,
    #[serde(rename = "Content")]
    pub content: String,
}

/// Request body for `POST job.api/cost`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Cost")]
pub struct AddJobCostRequest {
    /// The job number, e.g. "J000309".
    #[serde(rename = "Job")]
    pub job: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "Quantity")]
    pub quantity: String,
    #[serde(rename = "UnitCost")]
    pub unit_cost: String,
    #[serde(rename = "UnitPrice")]
    pub unit_price: String,
    #[serde(rename = "Billable", skip_serializing_if = "Option::is_none")]
    pub billable: Option<String>,
}

/// Request body for `PUT job.api/cost`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Cost")]
pub struct UpdateJobCostRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "Quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "UnitCost", skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<String>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
    #[serde(rename = "Billable", skip_serializing_if = "Option::is_none")]
    pub billable: Option<String>,
}

/// Request body for `PUT job.api/assign`.
///
/// The XPM API uses attribute-based XML (`<add uuid="..." />`, `<remove uuid="..." />`,
/// `<add-manager uuid="..." />`, etc.) which cannot be directly expressed via serde.
/// Callers should build the XML manually for this endpoint. This struct captures
/// the data needed.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Job")]
pub struct AssignJobRequest {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    /// Staff UUIDs to add to the job.
    #[serde(skip)]
    pub add: Vec<Uuid>,
    /// Staff UUIDs to remove from the job.
    #[serde(skip)]
    pub remove: Vec<Uuid>,
    /// Staff UUID to set as manager.
    #[serde(skip)]
    pub add_manager: Option<Uuid>,
    /// Whether to remove the current manager.
    #[serde(skip)]
    pub remove_manager: bool,
    /// Staff UUID to set as partner.
    #[serde(skip)]
    pub add_partner: Option<Uuid>,
    /// Whether to remove the current partner.
    #[serde(skip)]
    pub remove_partner: bool,
}

/// Request body for `POST job.api/applytemplate`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Job")]
pub struct ApplyTemplateRequest {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "TemplateUUID")]
    pub template_uuid: Uuid,
    #[serde(rename = "TaskMode", skip_serializing_if = "Option::is_none")]
    pub task_mode: Option<String>,
}
