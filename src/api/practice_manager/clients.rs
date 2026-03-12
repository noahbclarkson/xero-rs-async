//! API resource for XPM Clients (Practice Manager v3.1).

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::client::{
    ClientPaginatedResponse, ClientResponse, ClientsResponse, ContactResponse, ContactsResponse,
    CustomFieldsResponse, DocumentsResponse,
};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Practice Manager Clients.
#[derive(Debug, Clone, Copy)]
pub struct ClientsResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> ClientsResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    // ── Client CRUD ─────────────────────────────────────────────────────

    /// Return a list of all clients.
    ///
    /// * `detailed` – when `true`, returns the full detail for each client.
    /// * `modified_since` – optional UTC datetime string (`yyyy-MM-ddTHH:mm:ss`)
    ///   to filter clients modified after that point.
    pub async fn list(
        &self,
        detailed: bool,
        modified_since: Option<&str>,
    ) -> Result<ClientsResponse, XeroError> {
        let mut query = Vec::new();
        if detailed {
            query.push(("detailed".to_string(), "true".to_string()));
        }
        if let Some(ms) = modified_since {
            query.push(("modifiedsince".to_string(), ms.to_string()));
        }
        let q = if query.is_empty() {
            None
        } else {
            Some(query.as_slice())
        };
        self.api
            .client
            .send_request_xml(Method::GET, "/client.api/list", q)
            .await
    }

    /// Return a paginated subset of non-archived and non-deleted clients.
    ///
    /// * `page_size` – number of clients per page (1–500, default 50).
    /// * `page_token` – continuation token from a previous response.
    pub async fn list_paginated(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ClientPaginatedResponse, XeroError> {
        let mut query = Vec::new();
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        if let Some(pt) = page_token {
            query.push(("pageToken".to_string(), pt.to_string()));
        }
        let q = if query.is_empty() {
            None
        } else {
            Some(query.as_slice())
        };
        self.api
            .client
            .send_request_xml(Method::GET, "/client.api/paged-list", q)
            .await
    }

    /// Search clients by a query string.
    ///
    /// * `query` – the search term.
    /// * `detailed` – when `true`, returns the full detail for each client.
    pub async fn search(&self, query: &str, detailed: bool) -> Result<ClientsResponse, XeroError> {
        let mut params = vec![("query".to_string(), query.to_string())];
        if detailed {
            params.push(("detailed".to_string(), "true".to_string()));
        }
        self.api
            .client
            .send_request_xml(Method::GET, "/client.api/search", Some(params.as_slice()))
            .await
    }

    /// Retrieve detailed information for a specific client.
    pub async fn get(&self, uuid: Uuid) -> Result<ClientResponse, XeroError> {
        let path = format!("/client.api/get/{uuid}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Create a new client (and optionally add contacts to it).
    ///
    /// The caller supplies the full `<Client>` XML body.
    pub async fn add(&self, xml_body: &str) -> Result<ClientResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::POST, "/client.api/add", xml_body)
            .await
    }

    /// Update a client's details.
    ///
    /// The caller supplies the full `<Client>` XML body (must include `<UUID>`).
    pub async fn update(&self, xml_body: &str) -> Result<ClientResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/client.api/update", xml_body)
            .await
    }

    /// Archive a client by UUID.
    pub async fn archive(&self, uuid: Uuid) -> Result<ClientResponse, XeroError> {
        let body = format!("<Client><UUID>{uuid}</UUID></Client>");
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/client.api/archive", &body)
            .await
    }

    /// Delete a client by UUID.
    pub async fn delete(&self, uuid: Uuid) -> Result<(), XeroError> {
        let body = format!("<Client><UUID>{uuid}</UUID></Client>");
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/client.api/delete", Some(&body))
            .await
    }

    // ── Contacts ────────────────────────────────────────────────────────

    /// Return a paginated list of non-deleted contacts.
    ///
    /// * `page_size` – maximum contacts per page (1–500, default 50).
    /// * `page_token` – continuation token from a previous response.
    pub async fn list_contacts(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ContactsResponse, XeroError> {
        let mut query = Vec::new();
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        if let Some(pt) = page_token {
            query.push(("pageToken".to_string(), pt.to_string()));
        }
        let q = if query.is_empty() {
            None
        } else {
            Some(query.as_slice())
        };
        self.api
            .client
            .send_request_xml(Method::GET, "/client.api/contacts", q)
            .await
    }

    /// Retrieve detailed information for a specific contact.
    ///
    /// * `uuid` – the contact's UUID.
    /// * `client_uuid` – optional client UUID to guarantee `IsPrimary` and
    ///   `Position` are returned.
    pub async fn get_contact(
        &self,
        uuid: Uuid,
        client_uuid: Option<Uuid>,
    ) -> Result<ContactResponse, XeroError> {
        let path = format!("/client.api/contact/{uuid}");
        let mut query = Vec::new();
        if let Some(cu) = client_uuid {
            query.push(("clientUuid".to_string(), cu.to_string()));
        }
        let q = if query.is_empty() {
            None
        } else {
            Some(query.as_slice())
        };
        self.api
            .client
            .send_request_xml(Method::GET, &path, q)
            .await
    }

    /// Update a contact's details.
    ///
    /// The caller supplies the full `<Contact>` XML body.
    pub async fn update_contact(&self, uuid: Uuid, xml_body: &str) -> Result<(), XeroError> {
        let path = format!("/client.api/contact/{uuid}");
        self.api
            .client
            .send_request_xml_empty_response(Method::PUT, &path, Some(xml_body))
            .await
    }

    /// Create a new contact and add it to a client.
    ///
    /// The caller supplies the full `<Contact>` XML body (must include
    /// `<Client><UUID>...</UUID></Client>`).
    pub async fn add_contact(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/client.api/contact", Some(xml_body))
            .await
    }

    /// Delete a contact.
    ///
    /// * `uuid` – the contact's UUID.
    /// * `client_uuid` – optional client UUID to scope the deletion.
    pub async fn delete_contact(
        &self,
        uuid: Uuid,
        client_uuid: Option<Uuid>,
    ) -> Result<(), XeroError> {
        let path = if let Some(cu) = client_uuid {
            format!("/client.api/contact/{uuid}?clientUuid={cu}")
        } else {
            format!("/client.api/contact/{uuid}")
        };
        self.api
            .client
            .send_request_xml_empty_response(Method::DELETE, &path, None)
            .await
    }

    /// Add up to 10 existing contacts to a client.
    ///
    /// The caller supplies the `<Client><Contacts>...</Contacts></Client>` XML body.
    pub async fn add_contacts_to_client(
        &self,
        uuid: Uuid,
        xml_body: &str,
    ) -> Result<(), XeroError> {
        let path = format!("/client.api/client/{uuid}/contacts");
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, &path, Some(xml_body))
            .await
    }

    // ── Documents ───────────────────────────────────────────────────────

    /// Return a list of documents for a client.
    pub async fn list_documents(&self, uuid: Uuid) -> Result<DocumentsResponse, XeroError> {
        let path = format!("/client.api/documents/{uuid}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Add a document to a client.
    ///
    /// The caller supplies the full `<Document>` XML body.
    pub async fn add_document(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/client.api/document", Some(xml_body))
            .await
    }

    // ── Relationships ───────────────────────────────────────────────────

    /// Add a relationship between clients (Practice Manager only).
    ///
    /// The caller supplies the full `<Relationship>` XML body.
    pub async fn add_relationship(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(
                Method::POST,
                "/client.api/addrelationship",
                Some(xml_body),
            )
            .await
    }

    /// Update the relationship details between clients (Practice Manager only).
    ///
    /// The caller supplies the full `<Relationship>` XML body (must include `<UUID>`).
    pub async fn update_relationship(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(
                Method::PUT,
                "/client.api/updaterelationship",
                Some(xml_body),
            )
            .await
    }

    /// Delete a relationship between clients (Practice Manager only).
    ///
    /// The caller supplies the `<Relationship><UUID>...</UUID></Relationship>` XML body.
    pub async fn delete_relationship(&self, xml_body: &str) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(
                Method::POST,
                "/client.api/deleterelationship",
                Some(xml_body),
            )
            .await
    }

    // ── Custom Fields ───────────────────────────────────────────────────

    /// Retrieve custom field data for a specific client.
    pub async fn get_custom_fields(&self, uuid: Uuid) -> Result<CustomFieldsResponse, XeroError> {
        let path = format!("/client.api/get/{uuid}/customfield");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Update custom field data for a specific client.
    ///
    /// The caller supplies the `<CustomFields>` XML body.
    pub async fn update_custom_fields(&self, uuid: Uuid, xml_body: &str) -> Result<(), XeroError> {
        let path = format!("/client.api/update/{uuid}/customfield");
        self.api
            .client
            .send_request_xml_empty_response(Method::PUT, &path, Some(xml_body))
            .await
    }
}
