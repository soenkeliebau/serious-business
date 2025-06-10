use crate::baserow::field_types::TableField;
use crate::baserow::structs::BaserowOffer;
use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};

static LIST_TABLES_URL: &str = "https://api.baserow.io/api/database/tables/all-tables/";
static LIST_TABLE_FIELDS_URL: &str = "https://api.baserow.io/api/database/fields/table/";

struct Client {
    client: ReqwestClient,
}

#[derive(Serialize, Deserialize, Debug)]
struct Table {
    pub id: usize,
    pub name: String,
    pub order: usize,
    pub database_id: usize,
    pub fields: Option<Vec<TableField>>,
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

    pub async fn create_offer(&self, table: String, offer: BaserowOffer) {}

    pub async fn generate_structs(&self, database: usize) {
        let tablelist = self.list_tables().await;

        println!("use serde::{{Deserialize, Serialize}};\n");
        println!("trait BaserowObject {{");
        println!("  fn get_table_id(&self) -> usize;");
        println!("  fn get_id(&self) -> &str;");
        println!("}}\n\n");
        for table in tablelist {
            let struct_name = table.get_struct_name();
            let mut primary_field = "ERROR".to_string();
            println!("#[derive(Serialize, Deserialize, Debug)]");
            println!("pub struct {} {{", struct_name);
            if let Some(fields) = table.fields {
                let mut filtered_primary_fields = fields.iter().filter(|field| field.is_primary()).map(|field| field.get_name()).collect::<Vec<String>>();
                if filtered_primary_fields.len() != 1 {
                    panic!("got more or less than one primary field!");
                }
                primary_field = filtered_primary_fields.pop().unwrap();
                
                for field in fields {
                    println!(
                        "  {}: {},",
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
            println!("  fn get_id(&self) -> &str {{");
            println!("    &self.{}", primary_field);
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

    #[tokio::test]
    async fn test_create() {
        let client = Client::new("Q4kQ7u5MwSc2LqpAfFl03OS4FVzelr2Z");
        client.generate_structs(217366).await;
    }
}
