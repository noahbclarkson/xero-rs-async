use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::budget;
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Budgets.
#[derive(Debug, Clone, Copy)]
pub struct BudgetsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> BudgetsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list budgets.
    pub fn list(&self) -> BudgetsListRequest<'a> {
        BudgetsListRequest::new(self.api)
    }

    /// Retrieves a single budget by ID.
    pub async fn get(&self, budget_id: Uuid) -> Result<Vec<budget::Budget>, XeroError> {
        let path = format!("/Budgets/{budget_id}");
        let resp: budget::BudgetsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.budgets)
    }
}

/// Builder for Budgets list requests.
#[derive(Debug, Clone)]
pub struct BudgetsListRequest<'a> {
    api: &'a AccountingApi,
    date_to: Option<String>,
    date_from: Option<String>,
}

impl<'a> BudgetsListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            date_to: None,
            date_from: None,
        }
    }

    /// Filter budgets on the end date.
    pub fn date_to(mut self, date_to: impl Into<String>) -> Self {
        self.date_to = Some(date_to.into());
        self
    }

    /// Filter budgets on the start date.
    pub fn date_from(mut self, date_from: impl Into<String>) -> Self {
        self.date_from = Some(date_from.into());
        self
    }

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<budget::Budget>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("DateTo", self.date_to);
        query.push_opt_string("DateFrom", self.date_from);

        let resp: budget::BudgetsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Budgets", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.budgets)
    }
}

impl AccountingApi {
    /// Retrieves one or many budgets.
    pub async fn get_budgets(
        &self,
        budget_id: Option<Uuid>,
        date_to: Option<String>,
        date_from: Option<String>,
    ) -> Result<Vec<budget::Budget>, XeroError> {
        if let Some(id) = budget_id {
            self.budgets().get(id).await
        } else {
            let mut request = self.budgets().list();
            if let Some(date_to) = date_to {
                request = request.date_to(date_to);
            }
            if let Some(date_from) = date_from {
                request = request.date_from(date_from);
            }
            request.send().await
        }
    }
}
