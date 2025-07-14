use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use crate::bdwh::structs::TimeEntry as BdwhTimeEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: usize,
    pub label: String,
    pub description: Option<String>,
    #[serde(rename="type")]
    pub tag_type: Option<String>,
    pub status: isize,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryWrapper {
    pub timeEntry: TimeEntry
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    pub trackingType: TrackingType,
    pub validationStatus: ValidationStatus,
    pub day: NaiveDate,
    pub startTime: Option<NaiveTime>,
    pub endTime: Option<NaiveTime>,
    pub duration: usize,
    pub running: bool,
    pub comment: String,
    pub billedAt: Option<NaiveDate>,
    pub wasRejected: bool,
    pub firstSubmissionTime: Option<String>,
    pub overdueHours: Option<String>,
    pub approvedOn: Option<String>,
    pub createdAt: Option<String>,
    pub updatedAt: Option<String>,
    pub validation: Option<String>,
    pub project: String,
    pub task: String,
    pub user: String,
    pub team: String,
    pub timeEntryReference: Option<String>,
    pub reference: Option<String>,
    pub invoice: Option<String>,
    pub approvedBy: Option<bool>,
    pub sorting: Option<isize>,
    pub durationRoundedOverride: Option<usize>

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub createdAt: String,
    pub updatedAt: String,
    pub id: usize,
    pub status: usize,
    pub name: String,
    pub comment: Option<String>,
    pub code: String,
    pub billBy: usize,
    pub budgetBy: usize,
    pub hourlyRate: f64,
    pub budget: f64,
    pub budgetHours: f64,
    pub startDate: String,
    pub endDate: String,
    pub completedAt: Option<String>,
    pub roundingType: isize,
    pub roundingAmount: Option<isize>,
    pub progress: usize,
    pub projectState: usize,
    pub isSuperProject: bool,
    pub customField1: Option<String>,
    pub customField2: Option<String>,
    pub customField3: Option<String>,
    pub customField4: Option<String>,
    pub customField5: Option<String>,
    pub customField6: Option<String>,
    pub customField7: Option<String>,
    pub customField8: Option<String>,
    pub customField9: Option<String>,
    pub customField10: Option<String>,
    pub externalId: Option<String>,
    pub rating: usize,
    pub client: usize,
    pub color: usize,
    pub projectParent: Option<usize>
    
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
pub enum TrackingType {
    WORK
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
pub enum ValidationStatus {
    NOTSUBMITTED,
    APPROVED
}

impl From<BdwhTimeEntry> for TimeEntryWrapper {
    fn from(value: BdwhTimeEntry) -> Self {
        Self { timeEntry: value.into() }
    }
}

impl From<BdwhTimeEntry> for TimeEntry {
    fn from(value: BdwhTimeEntry) -> Self {
        let duration = value.total_time * 3600.0;
        let duration = duration.floor() as usize;
        Self{
            trackingType: TrackingType::WORK,
            validationStatus: ValidationStatus::APPROVED,
            day: value.date,
            startTime: None,
            endTime: None,
            duration,
            running: false,
            comment: value.comment,
            billedAt: None,
            wasRejected: false,
            firstSubmissionTime: None,
            overdueHours: None,
            approvedOn: None,
            createdAt: None,
            updatedAt: None,
            validation: None,
            project: value.cc_project,
            task: value.cc_task,
            user: value.cc_id,
            team: value.cc_team,
            timeEntryReference: None,
            reference: None,
            invoice: None,
            approvedBy: None,
            sorting: Some(1),
            durationRoundedOverride: None
        }
    }
}




