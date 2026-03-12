use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::{contact, contact_group};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Contact Groups.
#[derive(Debug, Clone, Copy)]
pub struct ContactGroupsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> ContactGroupsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list contact groups.
    pub fn list(&self) -> ContactGroupsListRequest<'a> {
        ContactGroupsListRequest::new(self.api)
    }

    /// Retrieves a single contact group by ID.
    pub async fn get(
        &self,
        contact_group_id: Uuid,
    ) -> Result<Vec<contact_group::ContactGroup>, XeroError> {
        let path = format!("/ContactGroups/{contact_group_id}");
        let resp: contact_group::ContactGroupsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.contact_groups)
    }

    /// Creates a new contact group.
    pub async fn create(
        &self,
        contact_group: contact_group::ContactGroup,
    ) -> Result<Vec<contact_group::ContactGroup>, XeroError> {
        let resp: contact_group::ContactGroupsResponse = self
            .api
            .client
            .send_request(Method::PUT, "/ContactGroups", None, Some(contact_group))
            .await?;
        Ok(resp.contact_groups)
    }

    /// Updates a contact group.
    pub async fn update(
        &self,
        contact_group_id: Uuid,
        contact_group_data: contact_group::ContactGroup,
    ) -> Result<Vec<contact_group::ContactGroup>, XeroError> {
        let path = format!("/ContactGroups/{contact_group_id}");
        let resp: contact_group::ContactGroupsResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(contact_group_data))
            .await?;
        Ok(resp.contact_groups)
    }

    /// Adds contacts to a contact group.
    pub async fn add_contacts(
        &self,
        contact_group_id: Uuid,
        contacts: Vec<contact::Contact>,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        let path = format!("/ContactGroups/{contact_group_id}/Contacts");
        let body = contact::ContactsRequest { contacts };
        let resp: contact::ContactsResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(body))
            .await?;
        Ok(resp.contacts)
    }

    /// Removes a specific contact from a contact group.
    pub async fn remove_contact(
        &self,
        contact_group_id: Uuid,
        contact_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/ContactGroups/{contact_group_id}/Contacts/{contact_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }

    /// Removes all contacts from a contact group.
    pub async fn remove_all_contacts(&self, contact_group_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/ContactGroups/{contact_group_id}/Contacts");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

/// Builder for Contact Groups list requests.
#[derive(Debug, Clone)]
pub struct ContactGroupsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> ContactGroupsListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            where_filter: None,
            order_by: None,
        }
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

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<contact_group::ContactGroup>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);

        let resp: contact_group::ContactGroupsResponse = self
            .api
            .client
            .send_request(Method::GET, "/ContactGroups", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.contact_groups)
    }
}

impl AccountingApi {
    /// Retrieves one or all contact groups.
    pub async fn get_contact_groups(
        &self,
        contact_group_id: Option<Uuid>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<contact_group::ContactGroup>, XeroError> {
        if let Some(id) = contact_group_id {
            self.contact_groups().get(id).await
        } else {
            let mut request = self.contact_groups().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            request.send().await
        }
    }

    /// Creates a new contact group.
    pub async fn create_contact_group(
        &self,
        contact_group: contact_group::ContactGroup,
    ) -> Result<Vec<contact_group::ContactGroup>, XeroError> {
        self.contact_groups().create(contact_group).await
    }

    /// Updates a contact group.
    pub async fn update_contact_group(
        &self,
        contact_group_id: Uuid,
        contact_group_data: contact_group::ContactGroup,
    ) -> Result<Vec<contact_group::ContactGroup>, XeroError> {
        self.contact_groups()
            .update(contact_group_id, contact_group_data)
            .await
    }

    /// Adds contacts to a contact group.
    pub async fn add_contacts_to_group(
        &self,
        contact_group_id: Uuid,
        contacts: Vec<contact::Contact>,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        self.contact_groups()
            .add_contacts(contact_group_id, contacts)
            .await
    }

    /// Removes a specific contact from a contact group.
    pub async fn remove_contact_from_group(
        &self,
        contact_group_id: Uuid,
        contact_id: Uuid,
    ) -> Result<(), XeroError> {
        self.contact_groups()
            .remove_contact(contact_group_id, contact_id)
            .await
    }

    /// Removes all contacts from a contact group.
    pub async fn remove_all_contacts_from_group(
        &self,
        contact_group_id: Uuid,
    ) -> Result<(), XeroError> {
        self.contact_groups()
            .remove_all_contacts(contact_group_id)
            .await
    }
}
