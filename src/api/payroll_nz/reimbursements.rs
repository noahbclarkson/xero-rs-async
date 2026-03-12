use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::reimbursement;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct ReimbursementsResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> ReimbursementsResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> ReimbursementsListRequest<'a> {
        ReimbursementsListRequest::new(self.api)
    }

    pub async fn get(
        &self,
        reimbursement_id: Uuid,
    ) -> Result<Vec<reimbursement::Reimbursement>, XeroError> {
        let path = format!("/reimbursements/{reimbursement_id}");
        let resp: reimbursement::ReimbursementsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.reimbursements.unwrap_or_default())
    }

    pub async fn create(
        &self,
        reimbursements: Vec<reimbursement::Reimbursement>,
    ) -> Result<Vec<reimbursement::Reimbursement>, XeroError> {
        let body = serde_json::json!({ "reimbursements": reimbursements });
        let resp: reimbursement::ReimbursementsResponse = self
            .api
            .client
            .send_request(Method::POST, "/reimbursements", None, Some(body))
            .await?;
        Ok(resp.reimbursements.unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct ReimbursementsListRequest<'a> {
    api: &'a PayrollNzApi,
    page: Option<u32>,
}

impl<'a> ReimbursementsListRequest<'a> {
    fn new(api: &'a PayrollNzApi) -> Self {
        Self { api, page: None }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<reimbursement::Reimbursement>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: reimbursement::ReimbursementsResponse = self
            .api
            .client
            .send_request(Method::GET, "/reimbursements", Some(&query), None::<()>)
            .await?;
        Ok(resp.reimbursements.unwrap_or_default())
    }
}
