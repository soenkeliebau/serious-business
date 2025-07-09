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
	
	https://stackable.coffeecup.app/v1/export/timeEntries?
	start_date=2025-06-01T00:00:00.000Z&
	end_date=2025-06-30T23:59:59.999Z&
	title=Hours Overview 01.06.2025 - 30.06.2025&
	duration_format=0&
	include_time_tracked=true&
	include_time_rounded=false&
	include_amount_rounded=false&
	columns[client]=true&
	columns[project]=true&
	columns[task]=true&
	columns[user]=true&
	columns[comment]=true&
	columns[team]=true&
	columns[reference]=true&
	columns[date]=true&
	columns[start_end]=false&
	group_by=date&
	show_only_summary_headers=false&
	format=pdf&
	access_token=da862de7c3ca55fc681319f7c99c543c10f15085
	
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
