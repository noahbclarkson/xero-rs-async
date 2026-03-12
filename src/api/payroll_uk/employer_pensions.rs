use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employer_pension;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployerPensionsResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployerPensionsResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> EmployerPensionsListRequest<'a> {
        EmployerPensionsListRequest::new(self.api)
    }

    pub async fn get(
        &self,
        employer_pension_id: Uuid,
    ) -> Result<employer_pension::EmployerPension, XeroError> {
        let path = format!("/benefits/{employer_pension_id}");
        let resp: employer_pension::EmployerPensionResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.benefit.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Benefit not found in response".to_string(),
        })
    }

    pub async fn create(
        &self,
        benefit: employer_pension::EmployerPension,
    ) -> Result<employer_pension::EmployerPension, XeroError> {
        let resp: employer_pension::EmployerPensionResponse = self
            .api
            .client
            .send_request(Method::POST, "/benefits", None, Some(benefit))
            .await?;
        resp.benefit.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Benefit not found in response".to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct EmployerPensionsListRequest<'a> {
    api: &'a PayrollUkApi,
    page: Option<u32>,
}

impl<'a> EmployerPensionsListRequest<'a> {
    fn new(api: &'a PayrollUkApi) -> Self {
        Self { api, page: None }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<employer_pension::EmployerPension>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: employer_pension::EmployerPensionsResponse = self
            .api
            .client
            .send_request(Method::GET, "/benefits", Some(&query), None::<()>)
            .await?;
        Ok(resp.benefits.unwrap_or_default())
    }
}
