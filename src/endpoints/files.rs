//! Entry point for interacting with the Xero Files API.

use crate::auth::TokenSet;
use crate::client::XeroClient;
use crate::error::XeroError;
use crate::models::files::{
    association::{Association, AssociationCount, AssociationsResponse},
    file::{File, FilesResponse},
    folder::{Folder, FoldersResponse},
};
use log::{error, trace};
use reqwest::{multipart, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

const BASE_URL: &str = "https://api.xero.com/files.xro/1.0";

/// A handle to the Files API endpoints.
#[derive(Debug, Clone)]
pub struct FilesApi {
    client: XeroClient,
    token_override: Option<Arc<TokenSet>>,
}

/// Private helper methods for the Files API.
impl FilesApi {
    async fn send_request<R, B>(
        &self,
        method: Method,
        tenant_id: Uuid,
        path: &str,
        query: Option<&[(String, String)]>,
        body: Option<B>,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", BASE_URL, path);
        let access_token = if let Some(token) = &self.token_override {
            token.access_token.clone()
        } else {
            self.client.token_manager.get_access_token().await?
        };

        let mut builder = self
            .client
            .http_client
            .request(method, &url)
            .bearer_auth(access_token)
            .header("xero-tenant-id", tenant_id.to_string())
            .header("Accept", "application/json");

        if let Some(q) = query {
            builder = builder.query(q);
        }
        if let Some(b) = body {
            builder = builder.json(&b);
        }

        let _permit = self.client.rate_limiter.acquire_permit(tenant_id).await?;
        let response = builder.send().await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            serde_json::from_str::<R>(&response_text).map_err(|e| {
                error!("Failed to deserialize JSON response from {}: {}", url, e);
                trace!(
                    "Raw JSON response that failed to parse:\n---\n{}\n---",
                    response_text
                );
                XeroError::from(e)
            })
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Api { status, message })
        }
    }

    async fn send_request_empty_response<B>(
        &self,
        method: Method,
        tenant_id: Uuid,
        path: &str,
        body: Option<B>,
    ) -> Result<(), XeroError>
    where
        B: Serialize,
    {
        let url = format!("{}{}", BASE_URL, path);
        let access_token = if let Some(token) = &self.token_override {
            token.access_token.clone()
        } else {
            self.client.token_manager.get_access_token().await?
        };

        let mut builder = self
            .client
            .http_client
            .request(method, &url)
            .bearer_auth(access_token)
            .header("xero-tenant-id", tenant_id.to_string());

        if let Some(b) = body {
            builder = builder.json(&b);
        }

        let _permit = self.client.rate_limiter.acquire_permit(tenant_id).await?;
        let response = builder.send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Api { status, message })
        }
    }
}

impl FilesApi {
    pub(crate) fn new(client: XeroClient) -> Self {
        Self {
            client,
            token_override: None,
        }
    }

    pub(crate) fn with_token_override(mut self, token: Arc<TokenSet>) -> Self {
        self.token_override = Some(token);
        self
    }

    // --- Files ---
    /// Retrieves a list of files.
    pub async fn get_files(
        &self,
        tenant_id: Uuid,
        page_size: Option<u32>,
        page: Option<u32>,
        sort: Option<String>,
        direction: Option<String>,
    ) -> Result<Vec<File>, XeroError> {
        let mut query = Vec::new();
        if let Some(ps) = page_size {
            query.push(("pagesize".to_string(), ps.to_string()));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(s) = sort {
            query.push(("sort".to_string(), s));
        }
        if let Some(d) = direction {
            query.push(("direction".to_string(), d));
        }
        let resp: FilesResponse = self
            .send_request(Method::GET, tenant_id, "/Files", Some(&query), None::<()>)
            .await?;
        Ok(resp.items)
    }

    /// Retrieves a specific file by its ID.
    pub async fn get_file_by_id(&self, tenant_id: Uuid, file_id: Uuid) -> Result<File, XeroError> {
        let path = format!("/Files/{}", file_id);
        self.send_request(Method::GET, tenant_id, &path, None, None::<()>)
            .await
    }

    /// Downloads the content of a specific file.
    pub async fn get_file_content(
        &self,
        tenant_id: Uuid,
        file_id: Uuid,
    ) -> Result<Vec<u8>, XeroError> {
        let path = format!("/Files/{}/Content", file_id);
        let url = format!("{}{}", BASE_URL, path);
        let access_token = if let Some(token) = &self.token_override {
            token.access_token.clone()
        } else {
            self.client.token_manager.get_access_token().await?
        };
        let builder = self
            .client
            .http_client
            .get(&url)
            .bearer_auth(access_token)
            .header("xero-tenant-id", tenant_id.to_string());

        let _permit = self.client.rate_limiter.acquire_permit(tenant_id).await?;
        let response = builder.send().await?;

        if response.status().is_success() {
            Ok(response.bytes().await?.to_vec())
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Api { status, message })
        }
    }

    async fn upload_file_internal(
        &self,
        tenant_id: Uuid,
        path: &str,
        file_name: String,
        body: Vec<u8>,
    ) -> Result<File, XeroError> {
        let url = format!("{}{}", BASE_URL, path);
        let part = multipart::Part::bytes(body).file_name(file_name.clone());
        let form = multipart::Form::new().part("file", part);

        let access_token = if let Some(token) = &self.token_override {
            token.access_token.clone()
        } else {
            self.client.token_manager.get_access_token().await?
        };
        let builder = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(access_token)
            .header("xero-tenant-id", tenant_id.to_string())
            .header("Accept", "application/json")
            .multipart(form);

        let _permit = self.client.rate_limiter.acquire_permit(tenant_id).await?;
        let response = builder.send().await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            serde_json::from_str::<File>(&response_text).map_err(|e| {
                error!(
                    "Failed to deserialize file upload response from {}: {}",
                    url, e
                );
                trace!(
                    "Raw JSON response from file upload that failed to parse:\n---\n{}\n---",
                    response_text
                );
                XeroError::from(e)
            })
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Api { status, message })
        }
    }

    /// Uploads a file to the inbox.
    pub async fn upload_file(
        &self,
        tenant_id: Uuid,
        file_name: String,
        body: Vec<u8>,
    ) -> Result<File, XeroError> {
        self.upload_file_internal(tenant_id, "/Files", file_name, body)
            .await
    }

    /// Uploads a file to a specific folder.
    pub async fn upload_file_to_folder(
        &self,
        tenant_id: Uuid,
        folder_id: Uuid,
        file_name: String,
        body: Vec<u8>,
    ) -> Result<File, XeroError> {
        let path = format!("/Files/{}", folder_id);
        self.upload_file_internal(tenant_id, &path, file_name, body)
            .await
    }

    /// Updates a file's name or folder.
    pub async fn update_file(
        &self,
        tenant_id: Uuid,
        file_id: Uuid,
        new_name: Option<String>,
        new_folder_id: Option<Uuid>,
    ) -> Result<File, XeroError> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct UpdateFileRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(rename = "FolderId", skip_serializing_if = "Option::is_none")]
            folder_id: Option<Uuid>,
        }
        let body = UpdateFileRequest {
            name: new_name,
            folder_id: new_folder_id,
        };
        let path = format!("/Files/{}", file_id);
        self.send_request(Method::PUT, tenant_id, &path, None, Some(body))
            .await
    }

    /// Deletes a file.
    pub async fn delete_file(&self, tenant_id: Uuid, file_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/Files/{}", file_id);
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    // --- Folders ---
    /// Retrieves a list of all folders.
    pub async fn get_folders(
        &self,
        tenant_id: Uuid,
        sort: Option<String>,
    ) -> Result<Vec<Folder>, XeroError> {
        let mut query = Vec::new();
        if let Some(s) = sort {
            query.push(("sort".to_string(), s));
        }
        let resp: FoldersResponse = self
            .send_request(Method::GET, tenant_id, "/Folders", Some(&query), None::<()>)
            .await?;
        Ok(resp.folders)
    }

    /// Retrieves a specific folder by its ID.
    pub async fn get_folder_by_id(
        &self,
        tenant_id: Uuid,
        folder_id: Uuid,
    ) -> Result<Folder, XeroError> {
        let path = format!("/Folders/{}", folder_id);
        self.send_request(Method::GET, tenant_id, &path, None, None::<()>)
            .await
    }

    /// Creates a new folder.
    pub async fn create_folder(&self, tenant_id: Uuid, name: String) -> Result<Folder, XeroError> {
        #[derive(Serialize)]
        struct CreateFolderRequest {
            name: String,
        }
        let body = CreateFolderRequest { name };
        self.send_request(Method::POST, tenant_id, "/Folders", None, Some(body))
            .await
    }

    /// Updates a folder's name.
    pub async fn update_folder(
        &self,
        tenant_id: Uuid,
        folder_id: Uuid,
        name: String,
    ) -> Result<Folder, XeroError> {
        #[derive(Serialize)]
        struct UpdateFolderRequest {
            name: String,
        }
        let body = UpdateFolderRequest { name };
        let path = format!("/Folders/{}", folder_id);
        self.send_request(Method::PUT, tenant_id, &path, None, Some(body))
            .await
    }

    /// Deletes a folder.
    pub async fn delete_folder(&self, tenant_id: Uuid, folder_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/Folders/{}", folder_id);
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    // --- Associations ---
    /// Retrieves a list of associations for a particular file.
    pub async fn get_file_associations(
        &self,
        tenant_id: Uuid,
        file_id: Uuid,
    ) -> Result<Vec<Association>, XeroError> {
        let path = format!("/Files/{}/Associations", file_id);
        let resp: AssociationsResponse = self
            .send_request(Method::GET, tenant_id, &path, None, None::<()>)
            .await?;
        Ok(resp.associations)
    }

    /// Retrieves a list of associations for a particular object (e.g., an invoice).
    pub async fn get_object_associations(
        &self,
        tenant_id: Uuid,
        object_id: Uuid,
    ) -> Result<Vec<Association>, XeroError> {
        let path = format!("/Associations/{}", object_id);
        let resp: AssociationsResponse = self
            .send_request(Method::GET, tenant_id, &path, None, None::<()>)
            .await?;
        Ok(resp.associations)
    }

    /// Retrieves a count of associations for a list of objects.
    pub async fn get_associations_count(
        &self,
        tenant_id: Uuid,
        object_ids: Vec<Uuid>,
    ) -> Result<AssociationCount, XeroError> {
        let ids_str = object_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let query = vec![("ObjectIds".to_string(), ids_str)];
        self.send_request(
            Method::GET,
            tenant_id,
            "/Associations/Count",
            Some(&query),
            None::<()>,
        )
        .await
    }

    /// Creates an association between a file and an object.
    pub async fn create_association(
        &self,
        tenant_id: Uuid,
        file_id: Uuid,
        association: Association,
    ) -> Result<Association, XeroError> {
        let path = format!("/Files/{}/Associations", file_id);
        self.send_request(Method::POST, tenant_id, &path, None, Some(association))
            .await
    }

    /// Deletes an association.
    pub async fn delete_association(
        &self,
        tenant_id: Uuid,
        file_id: Uuid,
        association_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/Files/{}/Associations/{}", file_id, association_id);
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }
}
