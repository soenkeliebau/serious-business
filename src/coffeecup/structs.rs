use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use crate::bdwh::structs::TimeEntry as BdwhTimeEntry;


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




