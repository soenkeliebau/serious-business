use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;
use std::fmt::Write;
use std::str::FromStr;
use std::string::ToString;
use baserow_client::client::{BaserowObject, Identifier};
use strum_macros::{Display, EnumString};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Companies {
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
    pub state: Option<CompaniesState>,
    #[serde(rename = "field_4142010")]
    pub ki_eingabeaufforderung: Option<String>,
    #[serde(rename = "field_4167285")]
    pub easybill: Option<String>,
    #[serde(rename = "field_4459573")]
    pub formula: Option<String>,
    #[serde(rename = "field_4459795")]
    pub customer_issues: Option<String>,
    #[serde(rename = "field_4459796", deserialize_with = "usize_or_null")]
    pub count: Option<usize>,
}
#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
#[serde(tag = "value")]
pub enum CompaniesState {
    #[serde(rename = "new")]
    #[strum(serialize = "new")]
    New { color: String, id: usize },
    #[serde(rename = "offer in progress")]
    #[strum(serialize = "offer in progress")]
    OfferInProgress { color: String, id: usize },
    #[serde(rename = "customer")]
    #[strum(serialize = "customer")]
    Customer { color: String, id: usize },
}
impl BaserowObject for Companies {
    fn get_static_table_id() -> usize {
        520298usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text {
            id: Some(
                match &self.name {
                    None => "".to_string(),
                    Some(name) => name.to_string(),
                },
            ),
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4133237".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscriptions {
    #[serde(rename = "field_4133311")]
    pub ty: Option<SubscriptionsType>,
    #[serde(rename = "field_4133313")]
    pub active: Option<bool>,
    #[serde(rename = "field_4133333")]
    pub companies: Option<String>,
    #[serde(rename = "field_4134285")]
    pub prolongation_date: Option<String>,
    #[serde(rename = "field_4134292", deserialize_with = "float_or_null")]
    pub arr: Option<f64>,
    #[serde(rename = "field_4134297", deserialize_with = "usize_or_null")]
    pub payment_freq: Option<usize>,
    #[serde(rename = "field_4134371")]
    pub next_payment_date: Option<String>,
    #[serde(rename = "field_4135241", deserialize_with = "usize_or_null")]
    pub nodes: Option<usize>,
    #[serde(rename = "field_4165002")]
    pub start_date: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
#[serde(tag = "value")]
pub enum SubscriptionsType {
    #[serde(rename = "Business Subscription")]
    #[strum(serialize = "Business Subscription")]
    BusinessSubscription { color: String, id: usize },
    #[serde(rename = "Basic Subscription")]
    #[strum(serialize = "Basic Subscription")]
    BasicSubscription { color: String, id: usize },
    #[serde(rename = "24/7 Support")]
    #[strum(serialize = "24/7 Support")]
    a247Support { color: String, id: usize },
}
impl BaserowObject for Subscriptions {
    fn get_static_table_id() -> usize {
        520307usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text {
            id: Some(
                match &self.ty {
                    None => "".to_string(),
                    Some(name) => name.to_string(),
                },
            ),
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4133311".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Jira {
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
impl BaserowObject for Jira {
    fn get_static_table_id() -> usize {
        520652usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text {
            id: Some(
                match &self.jira_issue_id {
                    None => "".to_string(),
                    Some(name) => name.to_string(),
                },
            ),
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4136052".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Easybill {
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
impl BaserowObject for Easybill {
    fn get_static_table_id() -> usize {
        521681usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text {
            id: Some(
                match &self.name {
                    None => "".to_string(),
                    Some(name) => name.to_string(),
                },
            ),
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4144714".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issues {
    #[serde(rename = "field_4422144")]
    pub url: Option<String>,
    #[serde(rename = "field_4422145")]
    pub notes: Option<String>,
    #[serde(rename = "field_4422148")]
    pub companies: Option<String>,
    #[serde(rename = "field_4459722")]
    pub customer_issues: Option<String>,
}
impl BaserowObject for Issues {
    fn get_static_table_id() -> usize {
        552081usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text {
            id: Some(
                match &self.url {
                    None => "".to_string(),
                    Some(name) => name.to_string(),
                },
            ),
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4422144".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Users {
    #[serde(rename = "field_4459526")]
    pub username: Option<String>,
    #[serde(rename = "field_4459527")]
    pub email: Option<String>,
    #[serde(rename = "field_4459528")]
    pub password: Option<String>,
}
impl BaserowObject for Users {
    fn get_static_table_id() -> usize {
        556243usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text {
            id: Some(
                match &self.username {
                    None => "".to_string(),
                    Some(name) => name.to_string(),
                },
            ),
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4459526".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerIssues {
    #[serde(rename = "field_4459691", deserialize_with = "usize_or_null")]
    pub bla: Option<usize>,
    #[serde(rename = "field_4459692")]
    pub customer: Option<String>,
    #[serde(rename = "field_4459693")]
    pub issue: Option<String>,
    #[serde(rename = "field_4459723")]
    pub description: Option<String>,
}
impl BaserowObject for CustomerIssues {
    fn get_static_table_id() -> usize {
        556261usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::UnsignedNumber {
            id: self.bla,
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4459691".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateDim {
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
impl BaserowObject for DateDim {
    fn get_static_table_id() -> usize {
        563399usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text {
            id: Some(
                match &self.day {
                    None => "".to_string(),
                    Some(name) => name.to_string(),
                },
            ),
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4520320".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FjContentPlan {
    #[serde(rename = "field_4520338")]
    pub title: Option<String>,
    #[serde(rename = "field_4520345")]
    pub status: Option<FjContentPlanStatus>,
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
    pub channel: Option<FjContentPlanChannel>,
}
#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
#[serde(tag = "value")]
pub enum FjContentPlanStatus {
    #[serde(rename = "in work")]
    #[strum(serialize = "in work")]
    InWork { color: String, id: usize },
    #[serde(rename = "in feedback")]
    #[strum(serialize = "in feedback")]
    InFeedback { color: String, id: usize },
    #[serde(rename = "in sign-off")]
    #[strum(serialize = "in sign-off")]
    InSignOff { color: String, id: usize },
    #[serde(rename = "published")]
    #[strum(serialize = "published")]
    Published { color: String, id: usize },
    #[serde(rename = "cancelled")]
    #[strum(serialize = "cancelled")]
    Cancelled { color: String, id: usize },
}
#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
#[serde(tag = "value")]
pub enum FjContentPlanChannel {
    #[serde(rename = "LinkedIn post")]
    #[strum(serialize = "LinkedIn post")]
    LinkedInPost { color: String, id: usize },
    #[serde(rename = "LinkedIn ad")]
    #[strum(serialize = "LinkedIn ad")]
    LinkedInAd { color: String, id: usize },
    #[serde(rename = "Blog post")]
    #[strum(serialize = "Blog post")]
    BlogPost { color: String, id: usize },
    #[serde(rename = "Website content")]
    #[strum(serialize = "Website content")]
    WebsiteContent { color: String, id: usize },
    #[serde(rename = "Case Study")]
    #[strum(serialize = "Case Study")]
    CaseStudy { color: String, id: usize },
}
impl BaserowObject for FjContentPlan {
    fn get_static_table_id() -> usize {
        563401usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text {
            id: Some(
                match &self.title {
                    None => "".to_string(),
                    Some(name) => name.to_string(),
                },
            ),
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4520338".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FjOrder {
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
    pub ty: Option<FjOrderType>,
    #[serde(rename = "field_4536586")]
    pub subscription: Option<bool>,
    #[serde(rename = "field_4536697")]
    pub offer: Option<String>,
    #[serde(rename = "field_4536700")]
    pub order: Option<String>,
    #[serde(rename = "field_4536703")]
    pub coffee_cup: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
#[serde(tag = "value")]
pub enum FjOrderType {
    #[serde(rename = "Subscription")]
    #[strum(serialize = "Subscription")]
    Subscription { color: String, id: usize },
    #[serde(rename = "Other")]
    #[strum(serialize = "Other")]
    Other { color: String, id: usize },
}
impl BaserowObject for FjOrder {
    fn get_static_table_id() -> usize {
        565099usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text {
            id: Some(
                match &self.ban {
                    None => "".to_string(),
                    Some(name) => name.to_string(),
                },
            ),
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4536094".to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Offers {
    #[serde(rename = "field_4565570", deserialize_with = "usize_or_null")]
    pub easybill_id: Option<usize>,
    #[serde(rename = "field_4565571")]
    pub customer: Option<String>,
    #[serde(rename = "field_4565572", deserialize_with = "float_or_null")]
    pub amount: Option<f64>,
    #[serde(rename = "field_4565632")]
    pub status: Option<OffersStatus>,
}
#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
#[serde(tag = "value")]
pub enum OffersStatus {
    #[serde(rename = "draft")]
    #[strum(serialize = "draft")]
    Draft { color: String, id: usize },
    #[serde(rename = "review")]
    #[strum(serialize = "review")]
    Review { color: String, id: usize },
    #[serde(rename = "approved")]
    #[strum(serialize = "approved")]
    Approved { color: String, id: usize },
    #[serde(rename = "sent")]
    #[strum(serialize = "sent")]
    Sent { color: String, id: usize },
    #[serde(rename = "expired")]
    #[strum(serialize = "expired")]
    Expired { color: String, id: usize },
    #[serde(rename = "accepted")]
    #[strum(serialize = "accepted")]
    Accepted { color: String, id: usize },
}
impl BaserowObject for Offers {
    fn get_static_table_id() -> usize {
        568215usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::UnsignedNumber {
            id: self.easybill_id,
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4565570".to_string()
    }
}
fn isize_or_null<'de, D>(deserializer: D) -> Result<Option<isize>, D::Error>
where
    D: Deserializer<'de>,
{
    struct IsizeOrNull;
    impl<'de> Visitor<'de> for IsizeOrNull {
        type Value = Option<isize>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("number or null")
        }
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(isize::from_str(value).unwrap()))
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }
    deserializer.deserialize_any(IsizeOrNull)
}
fn usize_or_null<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    struct UsizeOrNull;
    impl<'de> Visitor<'de> for UsizeOrNull {
        type Value = Option<usize>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("number or null")
        }
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(usize::from_str(value).unwrap()))
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }
    deserializer.deserialize_any(UsizeOrNull)
}
fn float_or_null<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct FloatOrNull;
    impl<'de> Visitor<'de> for FloatOrNull {
        type Value = Option<f64>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("number or null")
        }
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(f64::from_str(value).unwrap()))
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }
    deserializer.deserialize_any(FloatOrNull)
}
