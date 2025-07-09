use chrono::{Date, DateTime, FixedOffset, NaiveDate, NaiveTime};
use serde::Deserialize;
use trino_rust_client::{Client as TrinoClient, Trino};




/*
Following materialized view can be used with this data structure

create or replace materialized view lakehouse.bdwh.project_migration as
select
	v.dys_date as date ,
	cast(v.total_time as double) as total_time,
	cast(v.billable_time as double) as billable_time,
	v.start_time ,
	v.end_time,
	p.cc_id,
	p.cc_team,
	b.cc_project,
	b.cc_task,
	b.ban,
	v.comment
from
	lakehouse.bdwh.vw_stc_report_times_bill_l3y v,
	gsheet."default".ban_cc b ,
	gsheet."default".persons p
where
	v.pma_person_code = p.code
	and v.por_bt_internal_order_id = b.ban
 */

#[derive(Trino, Debug)]
pub struct TimeEntry {
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub total_time: f64,
    pub billable_time: f64,
    pub cc_project: String,
    pub ban: String,
    pub cc_id: String,
    pub cc_team: String,
    pub cc_task: String,
    pub comment: String,
}

