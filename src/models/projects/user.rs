//! Models for Projects Users.

use crate::models::projects::project::Pagination;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ProjectUser {
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUsersResponse {
    pub pagination: Option<Pagination>,
    pub items: Option<Vec<ProjectUser>>,
}
