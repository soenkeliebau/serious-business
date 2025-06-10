use serde::{Deserialize, Serialize};

use crate::baserow::client::{BaserowObject, Identifier};

#[derive(Serialize, Deserialize, Debug)]
pub struct companies {
    #[serde(rename = "field_4133237")]
    pub name: String,
    #[serde(rename = "field_4133238")]
    pub notes: String,
    #[serde(rename = "field_4133239")]
    pub active: bool,
    #[serde(rename = "field_4133334")]
    pub subscriptions: String,
    #[serde(rename = "field_4135947")]
    pub arr: String,
    #[serde(rename = "field_4135948")]
    pub contract_url: String,
    #[serde(rename = "field_4136227")]
    pub tech_account_mgr: String,
    #[serde(rename = "field_4136228")]
    pub biz_account_mgr: String,
    #[serde(rename = "field_4140645")]
    pub state: String,
    #[serde(rename = "field_4142010")]
    pub ki_eingabeaufforderung: String,
    #[serde(rename = "field_4167285")]
    pub easybill: String,
    #[serde(rename = "field_4459573")]
    pub formula: String,
    #[serde(rename = "field_4459795")]
    pub customer_issues: String,
    #[serde(rename = "field_4459796")]
    pub count: String,
}


impl BaserowObject for companies {

    fn get_table_id(&self) -> usize {
        520298
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.name.to_string() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct subscriptions {
    #[serde(rename = "field_4133311")]
    pub ty: String,
    #[serde(rename = "field_4133313")]
    pub active: bool,
    #[serde(rename = "field_4133333")]
    pub companies: String,
    #[serde(rename = "field_4134285")]
    pub prolongation_date: String,
    #[serde(rename = "field_4134292")]
    pub arr: isize,
    #[serde(rename = "field_4134297")]
    pub payment_freq: isize,
    #[serde(rename = "field_4134371")]
    pub next_payment_date: String,
    #[serde(rename = "field_4135241")]
    pub nodes: isize,
    #[serde(rename = "field_4165002")]
    pub start_date: String,
}


impl BaserowObject for subscriptions {

    fn get_table_id(&self) -> usize {
        520307
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.ty.to_string() }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct jira {
    #[serde(rename = "field_4136052")]
    pub jira_issue_id: String,
    #[serde(rename = "field_4136053")]
    pub summary: String,
    #[serde(rename = "field_4136054")]
    pub description: String,
    #[serde(rename = "field_4136055")]
    pub assignee: String,
    #[serde(rename = "field_4136056")]
    pub reporter: String,
    #[serde(rename = "field_4136057")]
    pub labels: String,
    #[serde(rename = "field_4136058")]
    pub created_date: String,
    #[serde(rename = "field_4136059")]
    pub updated_date: String,
    #[serde(rename = "field_4136060")]
    pub resolved_date: String,
    #[serde(rename = "field_4136061")]
    pub due_date: String,
    #[serde(rename = "field_4136062")]
    pub state: String,
    #[serde(rename = "field_4136063")]
    pub project: String,
    #[serde(rename = "field_4136064")]
    pub issue_url: String,
}


impl BaserowObject for jira {

    fn get_table_id(&self) -> usize {
        520652
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.jira_issue_id.to_string() }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct easybill {
    #[serde(rename = "field_4144714")]
    pub name: String,
    #[serde(rename = "field_4144716")]
    pub active: bool,
    #[serde(rename = "field_4144717")]
    pub country: String,
    #[serde(rename = "field_4144777")]
    pub customer_id: String,
    #[serde(rename = "field_4167286")]
    pub companies: String,
}


impl BaserowObject for easybill {

    fn get_table_id(&self) -> usize {
        521681
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.name.to_string() }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct issues {
    #[serde(rename = "field_4422144")]
    pub url: String,
    #[serde(rename = "field_4422145")]
    pub notes: String,
    #[serde(rename = "field_4422148")]
    pub companies: String,
    #[serde(rename = "field_4459722")]
    pub customer_issues: String,
}


impl BaserowObject for issues {

    fn get_table_id(&self) -> usize {
        552081
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.url.to_string() }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct users {
    #[serde(rename = "field_4459526")]
    pub username: String,
    #[serde(rename = "field_4459527")]
    pub email: String,
    #[serde(rename = "field_4459528")]
    pub password: String,
}


impl BaserowObject for users {

    fn get_table_id(&self) -> usize {
        556243
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.username.to_string() }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct customerIssues {
    #[serde(rename = "field_4459691")]
    pub bla: isize,
    #[serde(rename = "field_4459692")]
    pub customer: String,
    #[serde(rename = "field_4459693")]
    pub issue: String,
    #[serde(rename = "field_4459723")]
    pub description: String,
}


impl BaserowObject for customerIssues {

    fn get_table_id(&self) -> usize {
        556261
    }
    fn get_id(&self) -> Identifier {
        Identifier::Numeric { id: self.bla }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct dateDim {
    #[serde(rename = "field_4520320")]
    pub day: String,
    #[serde(rename = "field_4520319")]
    pub day_1: String,
    #[serde(rename = "field_4520321")]
    pub dayof_the_week: String,
    #[serde(rename = "field_4520322")]
    pub calendar_week: String,
    #[serde(rename = "field_4520323")]
    pub endof_month: String,
    #[serde(rename = "field_4520324")]
    pub quarter: String,
    #[serde(rename = "field_4520325")]
    pub dayof_year: String,
    #[serde(rename = "field_4520326")]
    pub fiscal_year: String,
    #[serde(rename = "field_4520327")]
    pub special_day: String,
}


impl BaserowObject for dateDim {

    fn get_table_id(&self) -> usize {
        563399
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.day.to_string() }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct fjContentPlan {
    #[serde(rename = "field_4520338")]
    pub title: String,
    #[serde(rename = "field_4520345")]
    pub status: String,
    #[serde(rename = "field_4520348")]
    pub release_date: String,
    #[serde(rename = "field_4520356")]
    pub g_drive_doc_url: String,
    #[serde(rename = "field_4520357")]
    pub live_url: String,
    #[serde(rename = "field_4520403")]
    pub briefing: String,
    #[serde(rename = "field_4520526")]
    pub responsible: String,
    #[serde(rename = "field_4520626")]
    pub channel: String,
}


impl BaserowObject for fjContentPlan {

    fn get_table_id(&self) -> usize {
        563401
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.title.to_string() }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct fjOrder {
    #[serde(rename = "field_4536094")]
    pub ban: String,
    #[serde(rename = "field_4536095")]
    pub kunden_bestellnummer: String,
    #[serde(rename = "field_4536096")]
    pub auftragsname: String,
    #[serde(rename = "field_4536097")]
    pub unternehmen: String,
    #[serde(rename = "field_4536098")]
    pub kostenstelle: String,
    #[serde(rename = "field_4536099")]
    pub summe: String,
    #[serde(rename = "field_4536100")]
    pub währung: String,
    #[serde(rename = "field_4536101")]
    pub status: String,
    #[serde(rename = "field_4536102")]
    pub buchungsstart: String,
    #[serde(rename = "field_4536103")]
    pub buchungsende: String,
    #[serde(rename = "field_4536104")]
    pub erstelldatum: String,
    #[serde(rename = "field_4536105")]
    pub erstellt_von: String,
    #[serde(rename = "field_4536545")]
    pub ty: String,
    #[serde(rename = "field_4536586")]
    pub subscription: bool,
    #[serde(rename = "field_4536697")]
    pub offer: String,
    #[serde(rename = "field_4536700")]
    pub order: String,
    #[serde(rename = "field_4536703")]
    pub coffee_cup: String,
}


impl BaserowObject for fjOrder {

    fn get_table_id(&self) -> usize {
        565099
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text { id: self.ban.to_string() }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct offers {
    #[serde(rename = "field_4565570")]
    pub easybill_id: isize,
    #[serde(rename = "field_4565571")]
    pub customer: String,
    #[serde(rename = "field_4565572")]
    pub amount: isize,
    #[serde(rename = "field_4565632")]
    pub status: String,
}


impl BaserowObject for offers {

    fn get_table_id(&self) -> usize {
        568215
    }
    fn get_id(&self) -> Identifier {
        Identifier::Numeric { id: self.easybill_id }
    }
}
