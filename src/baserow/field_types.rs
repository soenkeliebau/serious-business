use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedFields {
    pub id: usize,
    pub table_id: usize,
    pub name: String,
    pub order: usize,
    pub primary: bool,
    pub read_only: bool,
    pub immutable_type: Option<bool>,
    pub immutable_properties: bool,
    pub description: Option<String>,
    pub database_id: usize,
    pub workspace_id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TableField {
    #[serde(rename = "text")]
    Text {
        #[serde(flatten)]
        shared_fields: SharedFields,
        text_default: Option<String>,
    },
    #[serde(rename = "long_text")]
    LongText {
        #[serde(flatten)]
        shared_fields: SharedFields,
        long_text_enable_rich_text: Option<bool>,
    },
    #[serde(rename = "url")]
    Url {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "email")]
    Email {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "number")]
    Number {
        #[serde(flatten)]
        shared_fields: SharedFields,
        number_decimal_places: isize,
        number_negative: bool,
        number_prefix: String,
        number_suffix: String,
        number_default: Option<String>,
    },
    #[serde(rename = "rating")]
    Rating {
        #[serde(flatten)]
        shared_fields: SharedFields,
        max_value: isize,
        color: String,
        style: String,
    },
    #[serde(rename = "boolean")]
    Boolean {
        #[serde(flatten)]
        shared_fields: SharedFields,
        boolean_default: bool,
    },
    #[serde(rename = "date")]
    Date {
        #[serde(flatten)]
        shared_fields: SharedFields,
        date_format: String,
        date_include_time: bool,
        date_time_format: String,
        date_show_tzinfo: bool,
        date_force_timezone: Option<String>,
    },
    #[serde(rename = "last_modified")]
    LastModified {
        #[serde(flatten)]
        shared_fields: SharedFields,
        date_format: String,
        date_include_time: bool,
        date_time_format: String,
        date_show_tzinfo: bool,
        date_force_timezone: Option<String>,
    },
    #[serde(rename = "last_modified_by")]
    LastModifiedBy {
        #[serde(flatten)]
        shared_fields: SharedFields,
        available_collaborators: Vec<Collaborator>,
    },
    #[serde(rename = "created_on")]
    CreatedOn {
        #[serde(flatten)]
        shared_fields: SharedFields,
        date_format: String,
        date_include_time: bool,
        date_time_format: String,
        date_show_tzinfo: bool,
        date_force_timezone: Option<String>,
    },
    #[serde(rename = "created_by")]
    CreatedBy {
        #[serde(flatten)]
        shared_fields: SharedFields,
        available_collaborators: Vec<Collaborator>,
    },
    #[serde(rename = "duration")]
    Duration {
        #[serde(flatten)]
        shared_fields: SharedFields,
        duration_format: String,
    },
    #[serde(rename = "link_row")]
    LinkRow {
        #[serde(flatten)]
        shared_fields: SharedFields,
        link_row_table_id: Option<isize>,
        link_row_related_field_id: Option<isize>,
        link_row_table: Option<isize>,
        link_row_related_field: Option<isize>,
        link_row_limit_selection_view_id: Option<isize>,
        link_row_table_primary_field: Box<TableField>,
        link_row_multiple_relationships: bool,
    }, 
    #[serde(rename = "file")]
    File {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "single_select")]
    SingleSelect {
        #[serde(flatten)]
        shared_fields: SharedFields,
        select_options: Vec<SelectOption>,
        single_select_default: Option<isize>,
    },
    #[serde(rename = "multiple_select")]
    MultipleSelect {
        #[serde(flatten)]
        shared_fields: SharedFields,
        select_options: Vec<SelectOption>,
        multiple_select_default: Option<Vec<isize>>,
    },
    #[serde(rename = "phone_number")]
    PhoneNumber {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "formula")]
    Formula {
        #[serde(flatten)]
        shared_fields: SharedFields,
        date_time_format: Option<String>,
        array_formula_type: Option<String>,
        date_include_time: Option<bool>,
        error: Option<String>,
        number_suffix: String,
        date_force_timezone: Option<String>,
        nullable: bool,
        number_separator: String,
        select_options: Option<Vec<SelectOption>>,
        date_show_tzinfo: Option<bool>,
        number_prefix: String,
        date_format: Option<String>,
        number_decimal_places: Option<isize>,
        available_collaborators: Option<Vec<Collaborator>>,
        duration_format: Option<String>,
        formula: String,
        formula_type: String,
    },
    #[serde(rename = "count")]
    Count {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "rollup")]
    Rollup {
        #[serde(flatten)]
        shared_fields: SharedFields,
        date_time_format: Option<String>,
        array_formula_type: Option<String>,
        date_include_time: Option<bool>,
        error: Option<String>,
        number_suffix: String,
        date_force_timezone: Option<String>,
        nullable: bool,
        number_separator: String,
        date_show_tzinfo: Option<bool>,
        number_prefix: String,
        date_format: Option<String>,
        number_decimal_places: Option<isize>,
        duration_format: Option<String>,
        through_field_id: Option<isize>,
        target_field_id: Option<isize>,
        rollup_function: String,
        formula_type: String,
    },
    #[serde(rename = "lookup")]
    Lookup {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "multiple_collaborators")]
    MultipleCollaborators {
        #[serde(flatten)]
        shared_fields: SharedFields,
        available_collaborators: Vec<Collaborator>,
        notify_user_when_added: bool,
    },
    #[serde(rename = "uuid")]
    Uuid {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "autonumber")]
    AutoNumber {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "password")]
    Password {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "ai")]
    Ai {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collaborator {
    id: isize,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectOption {
    id: isize,
    value: String,
    color: String,
}

impl TableField {
    fn clean_name(dirty_name: &str) -> String {
        let clean_name = dirty_name.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "");
        let clean_name = clean_name.to_case(Case::Snake);
        if clean_name.eq("type") {
            "ty".to_string()
        } else {
            clean_name
        }
    }
    
    pub fn get_name(&self) -> String {
        match self {
            TableField::Text { shared_fields, .. } => Self::clean_name(&shared_fields.name), 
            TableField::LongText { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Url { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Email { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Number { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Rating { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Boolean { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Date { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::LastModified { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::LastModifiedBy { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::CreatedOn { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::CreatedBy { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Duration { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::LinkRow { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::File { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::SingleSelect { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::MultipleSelect { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::PhoneNumber { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Formula { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Count { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Rollup { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Lookup { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::MultipleCollaborators { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Uuid { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::AutoNumber { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Password { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
            TableField::Ai { shared_fields, .. } => Self::clean_name(&shared_fields.name),  
        }
    }
    
    pub fn get_rust_type(&self) -> &str {
        match self {
            TableField::Text { .. } => "String",
            TableField::LongText { .. } => "String",
            TableField::Url { .. } => "String",
            TableField::Email { .. } => "String",
            TableField::Number { .. } => "isize",
            TableField::Rating { .. } => "String",
            TableField::Boolean { .. } => "bool",
            TableField::Date { .. } => "String",
            TableField::LastModified { .. } => "String",
            TableField::LastModifiedBy { .. } => "String",
            TableField::CreatedOn { .. } => "String",
            TableField::CreatedBy { .. } => "String",
            TableField::Duration { .. } => "String",
            TableField::LinkRow { .. } => "String",
            TableField::File { .. } => "String",
            TableField::SingleSelect { .. } => "String",
            TableField::MultipleSelect { .. } => "String",
            TableField::PhoneNumber { .. } => "String",
            TableField::Formula { .. } => "String",
            TableField::Count { .. } => "String",
            TableField::Rollup { .. } => "String",
            TableField::Lookup { .. } => "String",
            TableField::MultipleCollaborators { .. } => "String",
            TableField::Uuid { .. } => "String",
            TableField::AutoNumber { .. } => "isize",
            TableField::Password { .. } => "String",
            TableField::Ai { .. } => "String",
        }
    }

    pub fn is_primary(&self) -> bool {
        match self {
            TableField::Text { shared_fields, .. } => shared_fields.primary,
            TableField::LongText { shared_fields, .. } => shared_fields.primary,
            TableField::Url { shared_fields, .. } => shared_fields.primary,
            TableField::Email { shared_fields, .. } => shared_fields.primary,
            TableField::Number { shared_fields, .. } => shared_fields.primary,
            TableField::Rating { shared_fields, .. } => shared_fields.primary,
            TableField::Boolean { shared_fields, .. } => shared_fields.primary,
            TableField::Date { shared_fields, .. } => shared_fields.primary,
            TableField::LastModified { shared_fields, .. } => shared_fields.primary,
            TableField::LastModifiedBy { shared_fields, .. } => shared_fields.primary,
            TableField::CreatedOn { shared_fields, .. } => shared_fields.primary,
            TableField::CreatedBy { shared_fields, .. } => shared_fields.primary,
            TableField::Duration { shared_fields, .. } => shared_fields.primary,
            TableField::LinkRow { shared_fields, .. } => shared_fields.primary,
            TableField::File { shared_fields, .. } => shared_fields.primary,
            TableField::SingleSelect { shared_fields, .. } => shared_fields.primary,
            TableField::MultipleSelect { shared_fields, .. } => shared_fields.primary,
            TableField::PhoneNumber { shared_fields, .. } => shared_fields.primary,
            TableField::Formula { shared_fields, .. } => shared_fields.primary,
            TableField::Count { shared_fields, .. } => shared_fields.primary,
            TableField::Rollup { shared_fields, .. } => shared_fields.primary,
            TableField::Lookup { shared_fields, .. } => shared_fields.primary,
            TableField::MultipleCollaborators { shared_fields, .. } => shared_fields.primary,
            TableField::Uuid { shared_fields, .. } => shared_fields.primary,
            TableField::AutoNumber { shared_fields, .. } => shared_fields.primary,
            TableField::Password { shared_fields, .. } => shared_fields.primary,
            TableField::Ai { shared_fields, .. } => shared_fields.primary,
        }
    }

    pub fn get_id(&self) -> usize {
        match self {
            TableField::Text { shared_fields, .. } => shared_fields.id,
            TableField::LongText { shared_fields, .. } => shared_fields.id,
            TableField::Url { shared_fields, .. } => shared_fields.id,
            TableField::Email { shared_fields, .. } => shared_fields.id,
            TableField::Number { shared_fields, .. } => shared_fields.id,
            TableField::Rating { shared_fields, .. } => shared_fields.id,
            TableField::Boolean { shared_fields, .. } => shared_fields.id,
            TableField::Date { shared_fields, .. } => shared_fields.id,
            TableField::LastModified { shared_fields, .. } => shared_fields.id,
            TableField::LastModifiedBy { shared_fields, .. } => shared_fields.id,
            TableField::CreatedOn { shared_fields, .. } => shared_fields.id,
            TableField::CreatedBy { shared_fields, .. } => shared_fields.id,
            TableField::Duration { shared_fields, .. } => shared_fields.id,
            TableField::LinkRow { shared_fields, .. } => shared_fields.id,
            TableField::File { shared_fields, .. } => shared_fields.id,
            TableField::SingleSelect { shared_fields, .. } => shared_fields.id,
            TableField::MultipleSelect { shared_fields, .. } => shared_fields.id,
            TableField::PhoneNumber { shared_fields, .. } => shared_fields.id,
            TableField::Formula { shared_fields, .. } => shared_fields.id,
            TableField::Count { shared_fields, .. } => shared_fields.id,
            TableField::Rollup { shared_fields, .. } => shared_fields.id,
            TableField::Lookup { shared_fields, .. } => shared_fields.id,
            TableField::MultipleCollaborators { shared_fields, .. } => shared_fields.id,
            TableField::Uuid { shared_fields, .. } => shared_fields.id,
            TableField::AutoNumber { shared_fields, .. } => shared_fields.id,
            TableField::Password { shared_fields, .. } => shared_fields.id,
            TableField::Ai { shared_fields, .. } => shared_fields.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::baserow::field_types::TableField;
    use std::fs;

    #[test]
    fn test_deserialize() {
        let contents = fs::read_to_string("testdata/field_types1.json")
            .expect("Should have been able to read the file");

        let json: Vec<TableField> =
            serde_json::from_str(&contents).expect("file should be proper JSON");

        let contents = fs::read_to_string("testdata/field_types2.json")
            .expect("Should have been able to read the file");

        let json: Vec<TableField> =
            serde_json::from_str(&contents).expect("file should be proper JSON");
    }
}
