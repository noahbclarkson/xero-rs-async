use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::contact::{self, CISSettings, CISSettingsResponse};
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Contacts.
#[derive(Debug, Clone, Copy)]
pub struct ContactsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> ContactsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list contacts.
    pub fn list(&self) -> ContactsListRequest<'a> {
        ContactsListRequest::new(self.api)
    }

    /// Retrieves a single contact by ID.
    pub async fn get(&self, contact_id: Uuid) -> Result<Vec<contact::Contact>, XeroError> {
        let path = format!("/Contacts/{contact_id}");
        let resp: contact::ContactsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.contacts)
    }

    /// Creates one or more new contacts.
    pub async fn create(
        &self,
        contacts: Vec<contact::Contact>,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        let body = if contacts.len() == 1 {
            serde_json::to_value(contacts.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(contact::ContactsRequest { contacts })?
        };
        let resp: contact::ContactsResponse = self
            .api
            .client
            .send_request(Method::PUT, "/Contacts", None, Some(body))
            .await?;
        Ok(resp.contacts)
    }

    /// Updates an existing contact.
    pub async fn update(
        &self,
        contact_id: Uuid,
        contact_data: contact::Contact,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        let path = format!("/Contacts/{contact_id}");
        let resp: contact::ContactsResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(contact_data))
            .await?;
        Ok(resp.contacts)
    }

    /// Retrieves CIS settings for a contact (UK only).
    pub async fn cis_settings(&self, contact_id: Uuid) -> Result<Vec<CISSettings>, XeroError> {
        let path = format!("/Contacts/{contact_id}/CISSettings");
        let resp: CISSettingsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.cis_settings)
    }
}

/// Builder for Contacts list requests.
#[derive(Debug, Clone)]
pub struct ContactsListRequest<'a> {
    api: &'a AccountingApi,
    ids: Option<Vec<Uuid>>,
    where_filter: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
    include_archived: Option<bool>,
    summary_only: Option<bool>,
    search_term: Option<String>,
}

impl<'a> ContactsListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            ids: None,
            where_filter: None,
            order_by: None,
            page: None,
            page_size: None,
            include_archived: None,
            summary_only: None,
            search_term: None,
        }
    }

    /// Filter by a list of contact IDs.
    pub fn ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = Uuid>,
    {
        self.ids = Some(ids.into_iter().collect());
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

    /// Includes archived contacts.
    pub fn include_archived(mut self, include_archived: bool) -> Self {
        self.include_archived = Some(include_archived);
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

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<contact::Contact>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_csv("IDs", self.ids);
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);
        query.push_opt("pageSize", self.page_size);
        query.push_opt("includeArchived", self.include_archived);
        query.push_opt("summaryOnly", self.summary_only);
        query.push_opt_string("searchTerm", self.search_term);

        let resp: contact::ContactsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Contacts", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.contacts)
    }
}

impl AccountingApi {
    /// Retrieves one or many contacts.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_contacts(
        &self,
        contact_id: Option<Uuid>,
        ids: Option<Vec<Uuid>>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
        include_archived: Option<bool>,
        summary_only: Option<bool>,
        search_term: Option<String>,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        if let Some(id) = contact_id {
            self.contacts().get(id).await
        } else {
            let mut request = self.contacts().list();
            if let Some(ids) = ids {
                request = request.ids(ids);
            }
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            if let Some(page) = page {
                request = request.page(page);
            }
            if let Some(page_size) = page_size {
                request = request.page_size(page_size);
            }
            if let Some(include_archived) = include_archived {
                request = request.include_archived(include_archived);
            }
            if let Some(summary_only) = summary_only {
                request = request.summary_only(summary_only);
            }
            if let Some(search_term) = search_term {
                request = request.search_term(search_term);
            }
            request.send().await
        }
    }

    /// Creates one or more new contacts.
    pub async fn create_contacts(
        &self,
        contacts: Vec<contact::Contact>,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        self.contacts().create(contacts).await
    }

    /// Updates an existing contact.
    pub async fn update_contact(
        &self,
        contact_id: Uuid,
        contact_data: contact::Contact,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        self.contacts().update(contact_id, contact_data).await
    }

    /// Retrieves CIS settings for a contact (UK only).
    pub async fn get_contact_cis_settings(
        &self,
        contact_id: Uuid,
    ) -> Result<Vec<CISSettings>, XeroError> {
        self.contacts().cis_settings(contact_id).await
    }
}
