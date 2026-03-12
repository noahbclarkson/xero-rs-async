//! API client for the Australian Payroll API.

use crate::auth::TokenSet;
use crate::client::XeroClient;
use crate::http::ApiClient;
use std::sync::Arc;
use uuid::Uuid;

pub mod earnings_rates;
pub mod employees;
pub mod leave_applications;
pub mod pay_items;
pub mod pay_runs;
pub mod pay_slips;
pub mod payroll_calendars;
pub mod settings;
pub mod super_fund_products;
pub mod super_funds;
pub mod timesheets;

const BASE_URL_V1: &str = "https://api.xero.com/payroll.xro/1.0";
const BASE_URL_V2: &str = "https://api.xero.com/payroll.xro/2.0";

#[derive(Debug, Clone)]
pub struct PayrollAuApi {
    client_v1: ApiClient,
    client_v2: ApiClient,
}

impl PayrollAuApi {
    pub(crate) fn new(client: XeroClient, tenant_id: Uuid) -> Self {
        Self {
            client_v1: ApiClient::new(
                BASE_URL_V1,
                tenant_id,
                client.http_client.clone(),
                client.token_manager.clone(),
                client.rate_limiter.clone(),
            ),
            client_v2: ApiClient::new(
                BASE_URL_V2,
                tenant_id,
                client.http_client.clone(),
                client.token_manager.clone(),
                client.rate_limiter.clone(),
            ),
        }
    }

    pub(crate) fn with_token_override(mut self, token: Arc<TokenSet>) -> Self {
        self.client_v1 = self.client_v1.with_token_override(token.clone());
        self.client_v2 = self.client_v2.with_token_override(token);
        self
    }

    #[must_use]
    pub fn employees(&self) -> employees::EmployeesResource<'_> {
        employees::EmployeesResource::new(self)
    }

    #[must_use]
    pub fn earnings_rates(&self) -> earnings_rates::EarningsRatesResource<'_> {
        earnings_rates::EarningsRatesResource::new(self)
    }

    #[must_use]
    pub fn pay_runs(&self) -> pay_runs::PayRunsResource<'_> {
        pay_runs::PayRunsResource::new(self)
    }

    #[must_use]
    pub fn pay_items(&self) -> pay_items::PayItemsResource<'_> {
        pay_items::PayItemsResource::new(self)
    }

    #[must_use]
    pub fn payroll_calendars(&self) -> payroll_calendars::PayrollCalendarsResource<'_> {
        payroll_calendars::PayrollCalendarsResource::new(self)
    }

    #[must_use]
    pub fn pay_slips(&self) -> pay_slips::PaySlipsResource<'_> {
        pay_slips::PaySlipsResource::new(self)
    }

    #[must_use]
    pub fn timesheets(&self) -> timesheets::TimesheetsResource<'_> {
        timesheets::TimesheetsResource::new(self)
    }

    #[must_use]
    pub fn leave_applications(&self) -> leave_applications::LeaveApplicationsResource<'_> {
        leave_applications::LeaveApplicationsResource::new(self)
    }

    #[must_use]
    pub fn settings(&self) -> settings::SettingsResource<'_> {
        settings::SettingsResource::new(self)
    }

    #[must_use]
    pub fn super_funds(&self) -> super_funds::SuperFundsResource<'_> {
        super_funds::SuperFundsResource::new(self)
    }

    #[must_use]
    pub fn super_fund_products(&self) -> super_fund_products::SuperFundProductsResource<'_> {
        super_fund_products::SuperFundProductsResource::new(self)
    }
}
