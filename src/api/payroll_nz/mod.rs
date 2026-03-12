//! API client for the New Zealand Payroll API.

use crate::auth::TokenSet;
use crate::client::XeroClient;
use crate::http::ApiClient;
use std::sync::Arc;
use uuid::Uuid;

pub mod deductions;
pub mod earnings_rates;
pub mod employee_leave_periods;
pub mod employee_leave_setup;
pub mod employee_leave_types;
pub mod employee_opening_balances;
pub mod employee_pay_templates;
pub mod employee_tax;
pub mod employee_working_patterns;
pub mod employees;
pub mod employment;
pub mod leave;
pub mod leave_balances;
pub mod leave_types;
pub mod pay_run_calendars;
pub mod pay_runs;
pub mod pay_slips;
pub mod payment_methods;
pub mod reimbursements;
pub mod salary_and_wages;
pub mod settings;
pub mod statutory_deductions;
pub mod superannuation;
pub mod timesheets;
pub mod tracking_categories;

const BASE_URL: &str = "https://api.xero.com/payroll.xro/2.0";

#[derive(Debug, Clone)]
pub struct PayrollNzApi {
    client: ApiClient,
}

impl PayrollNzApi {
    pub(crate) fn new(client: XeroClient, tenant_id: Uuid) -> Self {
        Self {
            client: ApiClient::new(
                BASE_URL,
                tenant_id,
                client.http_client.clone(),
                client.token_manager.clone(),
                client.rate_limiter.clone(),
            ),
        }
    }

    pub(crate) fn with_token_override(mut self, token: Arc<TokenSet>) -> Self {
        self.client = self.client.with_token_override(token);
        self
    }

    #[must_use]
    pub fn employees(&self) -> employees::EmployeesResource<'_> {
        employees::EmployeesResource::new(self)
    }

    #[must_use]
    pub fn deductions(&self) -> deductions::DeductionsResource<'_> {
        deductions::DeductionsResource::new(self)
    }

    #[must_use]
    pub fn earnings_rates(&self) -> earnings_rates::EarningsRatesResource<'_> {
        earnings_rates::EarningsRatesResource::new(self)
    }

    #[must_use]
    pub fn leave(&self) -> leave::LeaveResource<'_> {
        leave::LeaveResource::new(self)
    }

    #[must_use]
    pub fn leave_balances(&self) -> leave_balances::LeaveBalancesResource<'_> {
        leave_balances::LeaveBalancesResource::new(self)
    }

    #[must_use]
    pub fn leave_types(&self) -> leave_types::LeaveTypesResource<'_> {
        leave_types::LeaveTypesResource::new(self)
    }

    #[must_use]
    pub fn employee_leave_periods(
        &self,
    ) -> employee_leave_periods::EmployeeLeavePeriodsResource<'_> {
        employee_leave_periods::EmployeeLeavePeriodsResource::new(self)
    }

    #[must_use]
    pub fn employee_leave_setup(&self) -> employee_leave_setup::EmployeeLeaveSetupResource<'_> {
        employee_leave_setup::EmployeeLeaveSetupResource::new(self)
    }

    #[must_use]
    pub fn employee_leave_types(&self) -> employee_leave_types::EmployeeLeaveTypesResource<'_> {
        employee_leave_types::EmployeeLeaveTypesResource::new(self)
    }

    #[must_use]
    pub fn employee_opening_balances(
        &self,
    ) -> employee_opening_balances::EmployeeOpeningBalancesResource<'_> {
        employee_opening_balances::EmployeeOpeningBalancesResource::new(self)
    }

    #[must_use]
    pub fn employee_pay_templates(
        &self,
    ) -> employee_pay_templates::EmployeePayTemplatesResource<'_> {
        employee_pay_templates::EmployeePayTemplatesResource::new(self)
    }

    #[must_use]
    pub fn employee_tax(&self) -> employee_tax::EmployeeTaxResource<'_> {
        employee_tax::EmployeeTaxResource::new(self)
    }

    #[must_use]
    pub fn employee_working_patterns(
        &self,
    ) -> employee_working_patterns::EmployeeWorkingPatternsResource<'_> {
        employee_working_patterns::EmployeeWorkingPatternsResource::new(self)
    }

    #[must_use]
    pub fn employment(&self) -> employment::EmploymentResource<'_> {
        employment::EmploymentResource::new(self)
    }

    #[must_use]
    pub fn pay_runs(&self) -> pay_runs::PayRunsResource<'_> {
        pay_runs::PayRunsResource::new(self)
    }

    #[must_use]
    pub fn pay_run_calendars(&self) -> pay_run_calendars::PayRunCalendarsResource<'_> {
        pay_run_calendars::PayRunCalendarsResource::new(self)
    }

    #[must_use]
    pub fn payment_methods(&self) -> payment_methods::PaymentMethodsResource<'_> {
        payment_methods::PaymentMethodsResource::new(self)
    }

    #[must_use]
    pub fn timesheets(&self) -> timesheets::TimesheetsResource<'_> {
        timesheets::TimesheetsResource::new(self)
    }

    #[must_use]
    pub fn salary_and_wages(&self) -> salary_and_wages::SalaryAndWagesResource<'_> {
        salary_and_wages::SalaryAndWagesResource::new(self)
    }

    #[must_use]
    pub fn settings(&self) -> settings::SettingsResource<'_> {
        settings::SettingsResource::new(self)
    }

    #[must_use]
    pub fn pay_slips(&self) -> pay_slips::PaySlipsResource<'_> {
        pay_slips::PaySlipsResource::new(self)
    }

    #[must_use]
    pub fn reimbursements(&self) -> reimbursements::ReimbursementsResource<'_> {
        reimbursements::ReimbursementsResource::new(self)
    }

    #[must_use]
    pub fn superannuation(&self) -> superannuation::SuperannuationResource<'_> {
        superannuation::SuperannuationResource::new(self)
    }

    #[must_use]
    pub fn statutory_deductions(&self) -> statutory_deductions::StatutoryDeductionsResource<'_> {
        statutory_deductions::StatutoryDeductionsResource::new(self)
    }

    #[must_use]
    pub fn tracking_categories(&self) -> tracking_categories::TrackingCategoriesResource<'_> {
        tracking_categories::TrackingCategoriesResource::new(self)
    }
}
