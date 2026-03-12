//! Task (global definition) models for the XPM Practice Manager API v3.1.

use serde::Deserialize;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// `GET task.api/list` — list of all global task definitions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct TaskListResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "TaskList")]
    pub task_list: Option<TaskListInner>,
}

/// Inner wrapper for `<TaskList>` containing many `<Task>` elements.
#[derive(Debug, Clone, Deserialize)]
pub struct TaskListInner {
    #[serde(rename = "Task", default)]
    pub items: Vec<Task>,
}

/// `GET task.api/get/[uuid]` — single task definition.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct TaskResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Task")]
    pub task: Option<Task>,
}

// ---------------------------------------------------------------------------
// Main struct
// ---------------------------------------------------------------------------

/// A global task definition in XPM.
#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}
