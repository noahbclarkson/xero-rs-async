//! Models for Payroll UK Salary and Wages.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type SalaryAndWages = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalaryAndWagesResponse {
    pub salary_and_wages: Option<Vec<SalaryAndWages>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalaryAndWagesItemResponse {
    pub salary_and_wages: Option<SalaryAndWages>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalaryAndWagesRequest {
    pub salary_and_wages: SalaryAndWages,
}
