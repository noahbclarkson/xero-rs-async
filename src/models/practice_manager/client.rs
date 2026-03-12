//! Client models for the XPM Practice Manager API v3.1.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::{ClientRef, Pagination, StaffRef, YesNo};

// Re-export CustomFieldsResponse so the API layer can import it from `client`.
pub use super::custom_field::CustomFieldsResponse;

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// `GET client.api/list` — list of clients.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct ClientsResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Clients")]
    pub clients: Option<ClientList>,
}

/// Inner wrapper for `<Clients>` containing many `<Client>` elements.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientList {
    #[serde(rename = "Client", default)]
    pub items: Vec<Client>,
}

/// `GET client.api/get/[uuid]` — single client detail.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct ClientResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "WebUrl")]
    pub web_url: Option<String>,
    #[serde(rename = "Client")]
    pub client: Option<Client>,
}

/// Paginated client list response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct ClientPaginatedResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Clients")]
    pub clients: Option<ClientList>,
    #[serde(rename = "Pagination")]
    pub pagination: Option<Pagination>,
}

/// Response for paginated contacts list.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct ContactsResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Contacts")]
    pub contacts: Option<ContactList>,
    #[serde(rename = "Pagination")]
    pub pagination: Option<Pagination>,
}

/// Inner wrapper for `<Contacts>` at the response level.
#[derive(Debug, Clone, Deserialize)]
pub struct ContactList {
    #[serde(rename = "Contact", default)]
    pub items: Vec<Contact>,
}

/// Response for single contact GET.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct ContactResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Contact")]
    pub contact: Option<Contact>,
}

/// Response for GET documents/[uuid].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Documents")]
pub struct DocumentsResponse {
    #[serde(rename = "Document", default)]
    pub items: Vec<Document>,
}

// ---------------------------------------------------------------------------
// Main Client struct
// ---------------------------------------------------------------------------

/// Full client record returned by the XPM API.
#[derive(Debug, Clone, Deserialize)]
pub struct Client {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,

    // Personal / individual fields
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Gender")]
    pub gender: Option<String>,
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,
    #[serde(rename = "MiddleName")]
    pub middle_name: Option<String>,
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,
    #[serde(rename = "OtherName")]
    pub other_name: Option<String>,
    #[serde(rename = "Email")]
    pub email: Option<String>,
    #[serde(rename = "DateOfBirth")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "DateOfDeath")]
    pub date_of_death: Option<String>,
    #[serde(rename = "PlaceOfBirthCity")]
    pub place_of_birth_city: Option<String>,
    #[serde(rename = "PlaceOfBirthCountryCode")]
    pub place_of_birth_country_code: Option<String>,

    // Physical address
    #[serde(rename = "Address")]
    pub address: Option<String>,
    #[serde(rename = "City")]
    pub city: Option<String>,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "PostCode")]
    pub post_code: Option<String>,
    #[serde(rename = "Country")]
    pub country: Option<String>,

    // Postal address
    #[serde(rename = "PostalAddress")]
    pub postal_address: Option<String>,
    #[serde(rename = "PostalCity")]
    pub postal_city: Option<String>,
    #[serde(rename = "PostalRegion")]
    pub postal_region: Option<String>,
    #[serde(rename = "PostalPostCode")]
    pub postal_post_code: Option<String>,
    #[serde(rename = "PostalCountry")]
    pub postal_country: Option<String>,

    // Contact info
    #[serde(rename = "Phone")]
    pub phone: Option<String>,
    #[serde(rename = "Fax")]
    pub fax: Option<String>,
    #[serde(rename = "Website")]
    pub website: Option<String>,
    #[serde(rename = "ReferralSource")]
    pub referral_source: Option<String>,
    #[serde(rename = "ExportCode")]
    pub export_code: Option<String>,
    #[serde(rename = "Industry")]
    pub industry: Option<String>,

    // Status flags
    #[serde(rename = "IsArchived")]
    pub is_archived: Option<YesNo>,
    #[serde(rename = "IsDeleted")]
    pub is_deleted: Option<YesNo>,

    // Nested references
    #[serde(rename = "AccountManager")]
    pub account_manager: Option<StaffRef>,
    #[serde(rename = "Type")]
    pub client_type: Option<ClientType>,
    #[serde(rename = "Contacts")]
    pub contacts: Option<ClientContactList>,
    #[serde(rename = "Notes")]
    pub notes: Option<ClientNoteList>,
    #[serde(rename = "BillingClient")]
    pub billing_client: Option<ClientRef>,

    // Practice Management fields
    #[serde(rename = "JobManager")]
    pub job_manager: Option<StaffRef>,
    #[serde(rename = "TaxNumber")]
    pub tax_number: Option<String>,
    #[serde(rename = "CompanyNumber")]
    pub company_number: Option<String>,
    #[serde(rename = "BusinessNumber")]
    pub business_number: Option<String>,
    #[serde(rename = "BusinessStructure")]
    pub business_structure: Option<String>,
    #[serde(rename = "BalanceMonth")]
    pub balance_month: Option<String>,
    #[serde(rename = "PrepareGST")]
    pub prepare_gst: Option<String>,
    #[serde(rename = "GSTRegistered")]
    pub gst_registered: Option<String>,
    #[serde(rename = "GSTPeriod")]
    pub gst_period: Option<String>,
    #[serde(rename = "GSTBasis")]
    pub gst_basis: Option<String>,
    #[serde(rename = "ProvisionalTaxBasis")]
    pub provisional_tax_basis: Option<String>,
    #[serde(rename = "ProvisionalTaxRatio")]
    pub provisional_tax_ratio: Option<String>,

    // NZ-specific
    #[serde(rename = "SignedTaxAuthority")]
    pub signed_tax_authority: Option<String>,
    #[serde(rename = "TaxAgent")]
    pub tax_agent: Option<String>,
    #[serde(rename = "AgencyStatus")]
    pub agency_status: Option<String>,
    #[serde(rename = "ReturnType")]
    pub return_type: Option<String>,

    // AU-specific
    #[serde(rename = "PrepareActivityStatement")]
    pub prepare_activity_statement: Option<String>,
    #[serde(rename = "PrepareTaxReturn")]
    pub prepare_tax_return: Option<String>,

    // Groups & relationships
    #[serde(rename = "Groups")]
    pub groups: Option<GroupRefList>,
    #[serde(rename = "Relationships")]
    pub relationships: Option<RelationshipList>,
}

// ---------------------------------------------------------------------------
// Nested list wrappers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct ClientContactList {
    #[serde(rename = "Contact", default)]
    pub items: Vec<Contact>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClientNoteList {
    #[serde(rename = "Note", default)]
    pub items: Vec<ClientNote>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GroupRefList {
    #[serde(rename = "Group", default)]
    pub items: Vec<GroupRef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipList {
    #[serde(rename = "Relationship", default)]
    pub items: Vec<Relationship>,
}

// ---------------------------------------------------------------------------
// Sub-structs
// ---------------------------------------------------------------------------

/// Client type / billing configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientType {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "CostMarkup")]
    pub cost_markup: Option<String>,
    #[serde(rename = "PaymentTerm")]
    pub payment_term: Option<String>,
    #[serde(rename = "PaymentDay")]
    pub payment_day: Option<String>,
}

/// A contact linked to a client.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contact {
    #[serde(rename = "UUID")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Salutation")]
    pub salutation: Option<String>,
    #[serde(rename = "Addressee")]
    pub addressee: Option<String>,
    #[serde(rename = "Mobile")]
    pub mobile: Option<String>,
    #[serde(rename = "Email")]
    pub email: Option<String>,
    #[serde(rename = "Phone")]
    pub phone: Option<String>,
    #[serde(rename = "IsDeleted")]
    pub is_deleted: Option<YesNo>,
    #[serde(rename = "IsPrimary")]
    pub is_primary: Option<YesNo>,
    #[serde(rename = "Position")]
    pub position: Option<String>,
}

/// A note attached to a client.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientNote {
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Text")]
    pub text: Option<String>,
    #[serde(rename = "Folder")]
    pub folder: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<String>,
}

/// A group reference inside a client record.
#[derive(Debug, Clone, Deserialize)]
pub struct GroupRef {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
}

/// A relationship between two clients.
#[derive(Debug, Clone, Deserialize)]
pub struct Relationship {
    #[serde(rename = "UUID")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "Type")]
    pub relationship_type: Option<String>,
    #[serde(rename = "RelatedClient")]
    pub related_client: Option<ClientRef>,
    #[serde(rename = "NumberOfShares")]
    pub number_of_shares: Option<String>,
    #[serde(rename = "Percentage")]
    pub percentage: Option<String>,
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,
}

/// A document linked to a client.
#[derive(Debug, Clone, Deserialize)]
pub struct Document {
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Text")]
    pub text: Option<String>,
    #[serde(rename = "Folder")]
    pub folder: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<String>,
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
}

// ---------------------------------------------------------------------------
// Request types (Serialize only)
// ---------------------------------------------------------------------------

/// Request body for `POST client.api/add`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Client")]
pub struct AddClientRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "MiddleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "DateOfDeath", skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<String>,
    #[serde(rename = "PlaceOfBirthCity", skip_serializing_if = "Option::is_none")]
    pub place_of_birth_city: Option<String>,
    #[serde(
        rename = "PlaceOfBirthCountryCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub place_of_birth_country_code: Option<String>,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "City", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "Region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "PostCode", skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(rename = "Country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "PostalAddress", skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<String>,
    #[serde(rename = "PostalCity", skip_serializing_if = "Option::is_none")]
    pub postal_city: Option<String>,
    #[serde(rename = "PostalRegion", skip_serializing_if = "Option::is_none")]
    pub postal_region: Option<String>,
    #[serde(rename = "PostalPostCode", skip_serializing_if = "Option::is_none")]
    pub postal_post_code: Option<String>,
    #[serde(rename = "PostalCountry", skip_serializing_if = "Option::is_none")]
    pub postal_country: Option<String>,
    #[serde(rename = "Phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "Fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(rename = "WebSite", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(rename = "ReferralSource", skip_serializing_if = "Option::is_none")]
    pub referral_source: Option<String>,
    #[serde(rename = "ExportCode", skip_serializing_if = "Option::is_none")]
    pub export_code: Option<String>,
    #[serde(rename = "AccountManagerUUID", skip_serializing_if = "Option::is_none")]
    pub account_manager_uuid: Option<Uuid>,
    #[serde(rename = "Contacts", skip_serializing_if = "Option::is_none")]
    pub contacts: Option<AddContactListRequest>,
    #[serde(rename = "BillingClientUUID", skip_serializing_if = "Option::is_none")]
    pub billing_client_uuid: Option<Uuid>,

    // Practice Management fields
    #[serde(rename = "FirstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "OtherName", skip_serializing_if = "Option::is_none")]
    pub other_name: Option<String>,
    #[serde(rename = "DateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "JobManagerUUID", skip_serializing_if = "Option::is_none")]
    pub job_manager_uuid: Option<Uuid>,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,
    #[serde(rename = "CompanyNumber", skip_serializing_if = "Option::is_none")]
    pub company_number: Option<String>,
    #[serde(rename = "BusinessNumber", skip_serializing_if = "Option::is_none")]
    pub business_number: Option<String>,
    #[serde(rename = "BusinessStructure", skip_serializing_if = "Option::is_none")]
    pub business_structure: Option<String>,
    #[serde(rename = "BalanceMonth", skip_serializing_if = "Option::is_none")]
    pub balance_month: Option<String>,
    #[serde(rename = "PrepareGST", skip_serializing_if = "Option::is_none")]
    pub prepare_gst: Option<String>,
    #[serde(rename = "GSTRegistered", skip_serializing_if = "Option::is_none")]
    pub gst_registered: Option<String>,
    #[serde(rename = "GSTPeriod", skip_serializing_if = "Option::is_none")]
    pub gst_period: Option<String>,
    #[serde(rename = "GSTBasis", skip_serializing_if = "Option::is_none")]
    pub gst_basis: Option<String>,
    #[serde(
        rename = "ProvisionalTaxBasis",
        skip_serializing_if = "Option::is_none"
    )]
    pub provisional_tax_basis: Option<String>,
    #[serde(
        rename = "ProvisionalTaxRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub provisional_tax_ratio: Option<String>,
    #[serde(rename = "SignedTaxAuthority", skip_serializing_if = "Option::is_none")]
    pub signed_tax_authority: Option<String>,
    #[serde(rename = "TaxAgent", skip_serializing_if = "Option::is_none")]
    pub tax_agent: Option<String>,
    #[serde(rename = "AgencyStatus", skip_serializing_if = "Option::is_none")]
    pub agency_status: Option<String>,
    #[serde(rename = "ReturnType", skip_serializing_if = "Option::is_none")]
    pub return_type: Option<String>,
    #[serde(
        rename = "PrepareActivityStatement",
        skip_serializing_if = "Option::is_none"
    )]
    pub prepare_activity_statement: Option<String>,
    #[serde(rename = "PrepareTaxReturn", skip_serializing_if = "Option::is_none")]
    pub prepare_tax_return: Option<String>,
}

/// Wrapper for contacts inside add/update client requests.
#[derive(Debug, Clone, Serialize)]
pub struct AddContactListRequest {
    #[serde(rename = "Contact")]
    pub items: Vec<AddContactRequest>,
}

/// Request body for `PUT client.api/update`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Client")]
pub struct UpdateClientRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "MiddleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "DateOfDeath", skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<String>,
    #[serde(rename = "PlaceOfBirthCity", skip_serializing_if = "Option::is_none")]
    pub place_of_birth_city: Option<String>,
    #[serde(
        rename = "PlaceOfBirthCountryCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub place_of_birth_country_code: Option<String>,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "City", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "Region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "PostCode", skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(rename = "Country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "PostalAddress", skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<String>,
    #[serde(rename = "PostalCity", skip_serializing_if = "Option::is_none")]
    pub postal_city: Option<String>,
    #[serde(rename = "PostalRegion", skip_serializing_if = "Option::is_none")]
    pub postal_region: Option<String>,
    #[serde(rename = "PostalPostCode", skip_serializing_if = "Option::is_none")]
    pub postal_post_code: Option<String>,
    #[serde(rename = "PostalCountry", skip_serializing_if = "Option::is_none")]
    pub postal_country: Option<String>,
    #[serde(rename = "Phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "Fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(rename = "WebSite", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(rename = "ReferralSource", skip_serializing_if = "Option::is_none")]
    pub referral_source: Option<String>,
    #[serde(rename = "AccountManagerUUID", skip_serializing_if = "Option::is_none")]
    pub account_manager_uuid: Option<Uuid>,
    #[serde(rename = "BillingClientUUID", skip_serializing_if = "Option::is_none")]
    pub billing_client_uuid: Option<Uuid>,

    // Practice Management fields
    #[serde(rename = "FirstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "OtherName", skip_serializing_if = "Option::is_none")]
    pub other_name: Option<String>,
    #[serde(rename = "DateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "JobManagerUUID", skip_serializing_if = "Option::is_none")]
    pub job_manager_uuid: Option<Uuid>,
    #[serde(rename = "TaxNumber", skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,
    #[serde(rename = "CompanyNumber", skip_serializing_if = "Option::is_none")]
    pub company_number: Option<String>,
    #[serde(rename = "BusinessNumber", skip_serializing_if = "Option::is_none")]
    pub business_number: Option<String>,
    #[serde(rename = "BusinessStructure", skip_serializing_if = "Option::is_none")]
    pub business_structure: Option<String>,
    #[serde(rename = "BalanceMonth", skip_serializing_if = "Option::is_none")]
    pub balance_month: Option<String>,
    #[serde(rename = "PrepareGST", skip_serializing_if = "Option::is_none")]
    pub prepare_gst: Option<String>,
    #[serde(rename = "GSTRegistered", skip_serializing_if = "Option::is_none")]
    pub gst_registered: Option<String>,
    #[serde(rename = "GSTPeriod", skip_serializing_if = "Option::is_none")]
    pub gst_period: Option<String>,
    #[serde(rename = "GSTBasis", skip_serializing_if = "Option::is_none")]
    pub gst_basis: Option<String>,
    #[serde(
        rename = "ProvisionalTaxBasis",
        skip_serializing_if = "Option::is_none"
    )]
    pub provisional_tax_basis: Option<String>,
    #[serde(
        rename = "ProvisionalTaxRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub provisional_tax_ratio: Option<String>,
    #[serde(rename = "SignedTaxAuthority", skip_serializing_if = "Option::is_none")]
    pub signed_tax_authority: Option<String>,
    #[serde(rename = "TaxAgent", skip_serializing_if = "Option::is_none")]
    pub tax_agent: Option<String>,
    #[serde(rename = "AgencyStatus", skip_serializing_if = "Option::is_none")]
    pub agency_status: Option<String>,
    #[serde(rename = "ReturnType", skip_serializing_if = "Option::is_none")]
    pub return_type: Option<String>,
    #[serde(
        rename = "PrepareActivityStatement",
        skip_serializing_if = "Option::is_none"
    )]
    pub prepare_activity_statement: Option<String>,
    #[serde(rename = "PrepareTaxReturn", skip_serializing_if = "Option::is_none")]
    pub prepare_tax_return: Option<String>,
}

/// Request body for `PUT client.api/archive`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Client")]
pub struct ArchiveClientRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

/// Request body for `POST client.api/delete`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Client")]
pub struct DeleteClientRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

/// Request body for `POST client.api/contact` — add a new contact.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Contact")]
pub struct AddContactRequest {
    #[serde(rename = "Client", skip_serializing_if = "Option::is_none")]
    pub client: Option<ClientUuidRef>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "IsPrimary", skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<YesNo>,
    #[serde(rename = "Salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: Option<String>,
    #[serde(rename = "Addressee", skip_serializing_if = "Option::is_none")]
    pub addressee: Option<String>,
    #[serde(rename = "Phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "Mobile", skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// Simple client UUID reference used in contact request bodies.
#[derive(Debug, Clone, Serialize)]
pub struct ClientUuidRef {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

/// Request body for `PUT client.api/contact/[uuid]` — update a contact.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Contact")]
pub struct UpdateContactRequest {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: Option<String>,
    #[serde(rename = "Addressee", skip_serializing_if = "Option::is_none")]
    pub addressee: Option<String>,
    #[serde(rename = "Mobile", skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "Phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "Client", skip_serializing_if = "Option::is_none")]
    pub client: Option<ClientUuidRef>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "IsPrimary", skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<YesNo>,
}

/// Request body for `POST client.api/document`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Document")]
pub struct AddDocumentRequest {
    #[serde(rename = "ClientUUID")]
    pub client_uuid: Uuid,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Folder", skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(rename = "FileName")]
    pub file_name: String,
    #[serde(rename = "Content")]
    pub content: String,
}

/// Request body for `POST client.api/addrelationship`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Relationship")]
pub struct AddRelationshipRequest {
    #[serde(rename = "ClientUUID")]
    pub client_uuid: Uuid,
    #[serde(rename = "RelatedClientUUID")]
    pub related_client_uuid: Uuid,
    #[serde(rename = "Type")]
    pub relationship_type: String,
    #[serde(rename = "NumberOfShares", skip_serializing_if = "Option::is_none")]
    pub number_of_shares: Option<String>,
    #[serde(rename = "Percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// Request body for `PUT client.api/updaterelationship`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Relationship")]
pub struct UpdateRelationshipRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "NumberOfShares", skip_serializing_if = "Option::is_none")]
    pub number_of_shares: Option<String>,
    #[serde(rename = "Percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// Request body for `POST client.api/deleterelationship`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Relationship")]
pub struct DeleteRelationshipRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

/// Contact ref for `POST client/[uuid]/contacts`.
#[derive(Debug, Clone, Serialize)]
pub struct AddContactToClientEntry {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "IsPrimary", skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<YesNo>,
}

/// Request body wrapper for `POST client/[uuid]/contacts`.
#[derive(Debug, Clone, Serialize)]
pub struct AddContactsToClientContactList {
    #[serde(rename = "Contact")]
    pub items: Vec<AddContactToClientEntry>,
}

/// Request body for `POST client/[uuid]/contacts`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Client")]
pub struct AddContactsToClientRequest {
    #[serde(rename = "Contacts")]
    pub contacts: AddContactsToClientContactList,
}
