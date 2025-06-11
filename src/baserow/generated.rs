use serde::{Deserialize, Serialize};

use crate::baserow::client::{BaserowObject, Identifier};

#[derive(Serialize, Deserialize, Debug)]
pub struct companies {
    #[serde(rename = "field_4133237")]
    pub name: Option<String>,
    #[serde(rename = "field_4133238")]
    pub notes: Option<String>,
    #[serde(rename = "field_4133239")]
    pub active: Option<bool>,
    #[serde(rename = "field_4133334")]
    pub subscriptions: Option<String>,
    #[serde(rename = "field_4135947")]
    pub arr: Option<String>,
    #[serde(rename = "field_4135948")]
    pub contract_url: Option<String>,
    #[serde(rename = "field_4136227")]
    pub tech_account_mgr: Option<String>,
    #[serde(rename = "field_4136228")]
    pub biz_account_mgr: Option<String>,
    #[serde(rename = "field_4140645")]
    pub state: Option<String>,
    #[serde(rename = "field_4142010")]
    pub ki_eingabeaufforderung: Option<String>,
    #[serde(rename = "field_4167285")]
    pub easybill: Option<String>,
    #[serde(rename = "field_4459573")]
    pub formula: Option<String>,
    #[serde(rename = "field_4459795")]
    pub customer_issues: Option<String>,
    #[serde(rename = "field_4459796")]
    pub count: Option<String>,
}


impl BaserowObject for companies {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        520298
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.name.to_string() }
    }
    fn get_table_id_field(&self) -> String {
        "field_4133237".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct subscriptions {
    #[serde(rename = "field_4133311")]
    pub ty: Option<String>,
    #[serde(rename = "field_4133313")]
    pub active: Option<bool>,
    #[serde(rename = "field_4133333")]
    pub companies: Option<String>,
    #[serde(rename = "field_4134285")]
    pub prolongation_date: Option<String>,
    #[serde(rename = "field_4134292")]
    pub arr: Option<isize>,
    #[serde(rename = "field_4134297")]
    pub payment_freq: Option<isize>,
    #[serde(rename = "field_4134371")]
    pub next_payment_date: Option<String>,
    #[serde(rename = "field_4135241")]
    pub nodes: Option<isize>,
    #[serde(rename = "field_4165002")]
    pub start_date: Option<String>,
}


impl BaserowObject for subscriptions {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        520307
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.ty.to_string() }
    }
    fn get_table_id_field(&self) -> String {
        "field_4133311".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct jira {
    #[serde(rename = "field_4136052")]
    pub jira_issue_id: Option<String>,
    #[serde(rename = "field_4136053")]
    pub summary: Option<String>,
    #[serde(rename = "field_4136054")]
    pub description: Option<String>,
    #[serde(rename = "field_4136055")]
    pub assignee: Option<String>,
    #[serde(rename = "field_4136056")]
    pub reporter: Option<String>,
    #[serde(rename = "field_4136057")]
    pub labels: Option<String>,
    #[serde(rename = "field_4136058")]
    pub created_date: Option<String>,
    #[serde(rename = "field_4136059")]
    pub updated_date: Option<String>,
    #[serde(rename = "field_4136060")]
    pub resolved_date: Option<String>,
    #[serde(rename = "field_4136061")]
    pub due_date: Option<String>,
    #[serde(rename = "field_4136062")]
    pub state: Option<String>,
    #[serde(rename = "field_4136063")]
    pub project: Option<String>,
    #[serde(rename = "field_4136064")]
    pub issue_url: Option<String>,
}


impl BaserowObject for jira {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        520652
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.jira_issue_id.to_string() }
    }
    fn get_table_id_field(&self) -> String {
        "field_4136052".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct easybill {
    #[serde(rename = "field_4144714")]
    pub name: Option<String>,
    #[serde(rename = "field_4144716")]
    pub active: Option<bool>,
    #[serde(rename = "field_4144717")]
    pub country: Option<String>,
    #[serde(rename = "field_4144777")]
    pub customer_id: Option<String>,
    #[serde(rename = "field_4167286")]
    pub companies: Option<String>,
}


impl BaserowObject for easybill {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        521681
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.name.to_string() }
    }
    fn get_table_id_field(&self) -> String {
        "field_4144714".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct issues {
    #[serde(rename = "field_4422144")]
    pub url: Option<String>,
    #[serde(rename = "field_4422145")]
    pub notes: Option<String>,
    #[serde(rename = "field_4422148")]
    pub companies: Option<String>,
    #[serde(rename = "field_4459722")]
    pub customer_issues: Option<String>,
}


impl BaserowObject for issues {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        552081
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.url.to_string() }
    }
    fn get_table_id_field(&self) -> String {
        "field_4422144".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct users {
    #[serde(rename = "field_4459526")]
    pub username: Option<String>,
    #[serde(rename = "field_4459527")]
    pub email: Option<String>,
    #[serde(rename = "field_4459528")]
    pub password: Option<String>,
}


impl BaserowObject for users {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        556243
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.username.to_string() }
    }
    fn get_table_id_field(&self) -> String {
        "field_4459526".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct customerIssues {
    #[serde(rename = "field_4459691")]
    pub bla: Option<isize>,
    #[serde(rename = "field_4459692")]
    pub customer: Option<String>,
    #[serde(rename = "field_4459693")]
    pub issue: Option<String>,
    #[serde(rename = "field_4459723")]
    pub description: Option<String>,
}


impl BaserowObject for customerIssues {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        556261
    }
    fn get_id(&self) -> Identifier {
        Identifier::Numeric { id: self.bla }
    }
    fn get_table_id_field(&self) -> String {
        "field_4459691".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct dateDim {
    #[serde(rename = "field_4520320")]
    pub day: Option<String>,
    #[serde(rename = "field_4520319")]
    pub day_1: Option<String>,
    #[serde(rename = "field_4520321")]
    pub dayof_the_week: Option<String>,
    #[serde(rename = "field_4520322")]
    pub calendar_week: Option<String>,
    #[serde(rename = "field_4520323")]
    pub endof_month: Option<String>,
    #[serde(rename = "field_4520324")]
    pub quarter: Option<String>,
    #[serde(rename = "field_4520325")]
    pub dayof_year: Option<String>,
    #[serde(rename = "field_4520326")]
    pub fiscal_year: Option<String>,
    #[serde(rename = "field_4520327")]
    pub special_day: Option<String>,
}


impl BaserowObject for dateDim {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        563399
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.day.to_string() }
    }
    fn get_table_id_field(&self) -> String {
        "field_4520320".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct fjContentPlan {
    #[serde(rename = "field_4520338")]
    pub title: Option<String>,
    #[serde(rename = "field_4520345")]
    pub status: Option<String>,
    #[serde(rename = "field_4520348")]
    pub release_date: Option<String>,
    #[serde(rename = "field_4520356")]
    pub g_drive_doc_url: Option<String>,
    #[serde(rename = "field_4520357")]
    pub live_url: Option<String>,
    #[serde(rename = "field_4520403")]
    pub briefing: Option<String>,
    #[serde(rename = "field_4520526")]
    pub responsible: Option<String>,
    #[serde(rename = "field_4520626")]
    pub channel: Option<String>,
}


impl BaserowObject for fjContentPlan {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        563401
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.title.to_string() }
    }
    fn get_table_id_field(&self) -> String {
        "field_4520338".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct fjOrder {
    #[serde(rename = "field_4536094")]
    pub ban: Option<String>,
    #[serde(rename = "field_4536095")]
    pub kunden_bestellnummer: Option<String>,
    #[serde(rename = "field_4536096")]
    pub auftragsname: Option<String>,
    #[serde(rename = "field_4536097")]
    pub unternehmen: Option<String>,
    #[serde(rename = "field_4536098")]
    pub kostenstelle: Option<String>,
    #[serde(rename = "field_4536099")]
    pub summe: Option<String>,
    #[serde(rename = "field_4536100")]
    pub währung: Option<String>,
    #[serde(rename = "field_4536101")]
    pub status: Option<String>,
    #[serde(rename = "field_4536102")]
    pub buchungsstart: Option<String>,
    #[serde(rename = "field_4536103")]
    pub buchungsende: Option<String>,
    #[serde(rename = "field_4536104")]
    pub erstelldatum: Option<String>,
    #[serde(rename = "field_4536105")]
    pub erstellt_von: Option<String>,
    #[serde(rename = "field_4536545")]
    pub ty: Option<String>,
    #[serde(rename = "field_4536586")]
    pub subscription: Option<bool>,
    #[serde(rename = "field_4536697")]
    pub offer: Option<String>,
    #[serde(rename = "field_4536700")]
    pub order: Option<String>,
    #[serde(rename = "field_4536703")]
    pub coffee_cup: Option<String>,
}


impl BaserowObject for fjOrder {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        565099
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.ban.to_string() }
    }
    fn get_table_id_field(&self) -> String {
        "field_4536094".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct offers {
    #[serde(rename = "field_4565570")]
    pub easybill_id: Option<isize>,
    #[serde(rename = "field_4565571")]
    pub customer: Option<String>,
    #[serde(rename = "field_4565572")]
    pub amount: Option<isize>,
    #[serde(rename = "field_4565632")]
    pub status: Option<String>,
}


impl BaserowObject for offers {

    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_static_table_id() -> usize {
        568215
    }
    fn get_id(&self) -> Identifier {
        Identifier::Numeric { id: self.easybill_id }
    }
    fn get_table_id_field(&self) -> String {
        "field_4565570".to_string()
    }
}
