use crate::baserow::field_types::TableField;
use crate::baserow::generated::offers;
use crate::baserow::structs::BaserowOffer;
use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};

static LIST_TABLES_URL: &str = "https://api.baserow.io/api/database/tables/all-tables/";
static LIST_TABLE_FIELDS_URL: &str = "https://api.baserow.io/api/database/fields/table/";
static CREATE_RECORD_URL: &str = "https://api.baserow.io/api/database/rows/table/";
struct Client {
    client: ReqwestClient,
}

pub trait BaserowObject {
    fn get_table_id(&self) -> usize;
    fn get_id(&self) -> Identifier;
}

#[derive(Serialize, Deserialize, Debug)]
struct Table {
    pub id: usize,
    pub name: String,
    pub order: usize,
    pub database_id: usize,
    pub fields: Option<Vec<TableField>>,
}

pub enum Identifier {
    Numeric { id: isize },
    Text { id: String },
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        match self {
            Identifier::Numeric { id } => id.to_string(),
            Identifier::Text { id } => id.clone(),
        }
    }
}

impl Table {
    pub fn extend_with_fields(&mut self, fields: Vec<TableField>) {
        self.fields = Some(fields);
    }

    pub fn get_struct_name(&self) -> String {
        let result = some_kind_of_uppercase_first_letter(&self.name);
        result.to_case(Case::Camel)
    }
}

impl Client {
    pub fn new(token: &str) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Token {}", token)).unwrap(),
        );
        default_headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());

        Self {
            client: ReqwestClient::builder()
                .default_headers(default_headers)
                .build()
                .unwrap(),
        }
    }

    async fn list_tables(&self) -> Vec<Table> {
        let mut tables = self
            .client
            .get(LIST_TABLES_URL)
            .send()
            .await
            .unwrap()
            .json::<Vec<Table>>()
            .await
            .unwrap();

        for table in &mut tables {
            if let Some(table_fields) = self.list_table_fields(&table.id).await {
                table.extend_with_fields(table_fields);
            }
        }

        tables
    }

    async fn list_table_fields(&self, table_id: &usize) -> Option<Vec<TableField>> {
        if let Ok(response) = self
            .client
            .get(format!("{LIST_TABLE_FIELDS_URL}{table_id}/"))
            .send()
            .await
        {
            Some(response.json::<Vec<TableField>>().await.unwrap())
        } else {
            None
        }
    }

    pub async fn create<T>(&self, obj: &T)
    where
        T: BaserowObject + Serialize,
    {
        let create_url = format!("{CREATE_RECORD_URL}{}/", obj.get_table_id().to_string());
        
        println!("{:?}", self.client);
        let request = self
            .client
            .post(create_url)
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(obj).unwrap())
            .build().unwrap();
        
        println!("Request\n{:?}", request);
        let response = self.client.execute(request)
            .await
            .unwrap();

        println!("{:?}", response);
        println!("{:?}", response.text().await.unwrap())
    }

    pub async fn generate_structs(&self, database: usize) {
        let tablelist = self.list_tables().await;

        println!("use serde::{{Deserialize, Serialize}};\n");
        println!("use crate::baserow::client::{{BaserowObject, Identifier}};\n");
        for table in tablelist {
            let struct_name = table.get_struct_name();
            let mut primary_field = ("ERROR".to_string(), "ERROR".to_string());
            println!("#[derive(Serialize, Deserialize, Debug)]");
            println!("pub struct {} {{", struct_name);
            if let Some(fields) = table.fields {
                let mut filtered_primary_fields = fields
                    .iter()
                    .filter(|field| field.is_primary())
                    .map(|field| {
                        (
                            field.get_name().to_string(),
                            field.get_rust_type().to_string(),
                        )
                    })
                    .collect::<Vec<(String, String)>>();
                if filtered_primary_fields.len() != 1 {
                    panic!("got more or less than one primary field!");
                }
                primary_field = filtered_primary_fields.pop().unwrap();

                for field in fields {
                    println!("#[serde(rename = \"field_{}\")]", field.get_id());
                    println!(
                        "  pub {}: {},",
                        field.get_name().to_case(Case::Snake),
                        field.get_rust_type()
                    );
                }
            }
            println!("}}\n\n");
            println!("impl BaserowObject for {} {{\n", struct_name);
            println!("  fn get_table_id(&self) -> usize {{");
            println!("    {}", table.id);
            println!("  }}");
            println!("  fn get_id(&self) -> Identifier {{");
            Identifier::Text {
                id: primary_field.0.to_string(),
            };
            if primary_field.1.eq("String") {
                println!(
                    "    Identifier::Text {{ id: self.{}.to_string() }}",
                    primary_field.0.to_string()
                );
            } else {
                println!(
                    "    Identifier::Numeric {{ id: self.{} }}",
                    primary_field.0.to_string()
                );
            }

            println!("  }}");
            println!("}}");
        }
    }
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

#[cfg(test)]
mod tests {
    use crate::baserow::client::Client;
    use crate::baserow::generated::offers;

    #[tokio::test]
    async fn generate_code() {
        let client = Client::new("Q4kQ7u5MwSc2LqpAfFl03OS4FVzelr2Z");
        client.generate_structs(217366).await;
    }

    #[tokio::test]
    async fn test_create_offer() {
        let client = Client::new("Q4kQ7u5MwSc2LqpAfFl03OS4FVzelr2Z");
        let offer = offers {
            easybill_id: 123,
            customer: "testcustomer1234".to_string(),
            amount: 12000,
            status: "draft".to_string(),
        };

        client.create(&offer).await;
    }
}
