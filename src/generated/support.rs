use baserow_client::client::{BaserowObject, Identifier};
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;
use std::fmt::Write;
use std::str::FromStr;
use std::string::ToString;
use strum_macros::{Display, EnumString};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    #[serde(rename = "field_4598182")]
    pub name: Option<String>,
    #[serde(rename = "field_4598183")]
    pub oncall: Option<ScheduleOncall>,
    #[serde(rename = "field_4598188")]
    pub start: Option<String>,
    #[serde(rename = "field_4598189")]
    pub end: Option<String>,
    #[serde(rename = "field_4598228")]
    pub escalation: Option<ScheduleEscalation>,
}
#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
#[serde(tag = "value")]
pub enum ScheduleOncall {
    #[serde(rename = "jim.halfpenny@stackable.tech")]
    #[strum(serialize = "jim.halfpenny@stackable.tech")]
    Jimhalfpennystackabletech { color: String, id: usize },
    #[serde(rename = "soenke.liebau@stackable.tech")]
    #[strum(serialize = "soenke.liebau@stackable.tech")]
    Soenkeliebaustackabletech { color: String, id: usize },
}
#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
#[serde(tag = "value")]
pub enum ScheduleEscalation {
    #[serde(rename = "jim.halfpenny@stackable.tech")]
    #[strum(serialize = "jim.halfpenny@stackable.tech")]
    Jimhalfpennystackabletech { color: String, id: usize },
    #[serde(rename = "soenke.liebau@stackable.tech")]
    #[strum(serialize = "soenke.liebau@stackable.tech")]
    Soenkeliebaustackabletech { color: String, id: usize },
}
impl BaserowObject for Schedule {
    fn get_static_table_id() -> usize {
        571778usize
    }
    fn get_table_id(&self) -> usize {
        Self::get_static_table_id()
    }
    fn get_id(&self) -> Identifier {
        Identifier::Text {
            id: Some(match &self.name {
                None => "".to_string(),
                Some(name) => name.to_string(),
            }),
        }
    }
    fn get_table_id_field(&self) -> String {
        "field_4598182".to_string()
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
