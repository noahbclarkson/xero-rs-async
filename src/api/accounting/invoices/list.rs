use super::super::query::QueryParams;
use super::super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::invoice;
use reqwest::Method;
use uuid::Uuid;

/// Builder for Invoice list requests.
#[derive(Debug, Clone)]
pub struct InvoicesListRequest<'a> {
    api: &'a AccountingApi,
    invoice_numbers: Option<Vec<String>>,
    contact_ids: Option<Vec<Uuid>>,
    statuses: Option<Vec<String>>,
    where_filter: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
    summary_only: Option<bool>,
    search_term: Option<String>,
    ids: Option<Vec<Uuid>>,
    created_by_my_app: Option<bool>,
    unitdp: Option<u8>,
}

impl<'a> InvoicesListRequest<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            invoice_numbers: None,
            contact_ids: None,
            statuses: None,
            where_filter: None,
            order_by: None,
            page: None,
            page_size: None,
            summary_only: None,
            search_term: None,
            ids: None,
            created_by_my_app: None,
            unitdp: None,
        }
    }

    /// Filter by a list of invoice IDs.
    pub fn ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = Uuid>,
    {
        self.ids = Some(ids.into_iter().collect());
        self
    }

    /// Filter by a list of invoice numbers.
    pub fn invoice_numbers<I, S>(mut self, invoice_numbers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.invoice_numbers = Some(invoice_numbers.into_iter().map(Into::into).collect());
        self
    }

    /// Filter by a list of contact IDs.
    pub fn contact_ids<I>(mut self, contact_ids: I) -> Self
    where
        I: IntoIterator<Item = Uuid>,
    {
        self.contact_ids = Some(contact_ids.into_iter().collect());
        self
    }

    /// Filter by a list of statuses.
    pub fn statuses<I, S>(mut self, statuses: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.statuses = Some(statuses.into_iter().map(Into::into).collect());
        self
    }

    /// Filter using the `where` query parameter.
    pub fn where_filter(mut self, filter: impl Into<String>) -> Self {
        self.where_filter = Some(filter.into());
        self
    }

    /// Order by a field.
    pub fn order_by(mut self, order: impl Into<String>) -> Self {
        self.order_by = Some(order.into());
        self
    }

    /// Sets the page number.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the page size.
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Returns a lightweight response.
    pub fn summary_only(mut self, summary_only: bool) -> Self {
        self.summary_only = Some(summary_only);
        self
    }

    /// Sets the search term.
    pub fn search_term(mut self, search_term: impl Into<String>) -> Self {
        self.search_term = Some(search_term.into());
        self
    }

    /// Limit results to invoices created by your app.
    pub fn created_by_my_app(mut self, created_by_my_app: bool) -> Self {
        self.created_by_my_app = Some(created_by_my_app);
        self
    }

    /// Set unit decimal places for line item unit amounts.
    pub fn unitdp(mut self, unitdp: u8) -> Self {
        self.unitdp = Some(unitdp);
        self
    }

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<invoice::Invoice>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_csv("IDs", self.ids);
        if let Some(numbers) = self.invoice_numbers {
            query.push_string("InvoiceNumbers", numbers.join(","));
        }
        query.push_opt_csv("ContactIDs", self.contact_ids);
        if let Some(statuses) = self.statuses {
            query.push_string("Statuses", statuses.join(","));
        }
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);
        query.push_opt("pageSize", self.page_size);
        query.push_opt("summaryOnly", self.summary_only);
        query.push_opt_string("SearchTerm", self.search_term);
        query.push_opt("createdByMyApp", self.created_by_my_app);
        query.push_opt("unitdp", self.unitdp);

        let resp: invoice::InvoicesResponse = self
            .api
            .client
            .send_request(Method::GET, "/Invoices", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.invoices)
    }
}
