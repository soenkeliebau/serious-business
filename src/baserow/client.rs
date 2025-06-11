use crate::baserow::field_types::TableField;
use convert_case::{Case, Casing};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

static LIST_TABLES_URL: &str = "https://api.baserow.io/api/database/tables/all-tables/";
static LIST_TABLE_FIELDS_URL: &str = "https://api.baserow.io/api/database/fields/table/";
static CREATE_RECORD_URL: &str = "https://api.baserow.io/api/database/rows/table/";
static LIST_RECORD_URL: &str = "https://api.baserow.io/api/database/rows/table/";

struct Client {
    client: ReqwestClient,
}

pub trait BaserowObject {
    fn get_static_table_id() -> usize;
    fn get_table_id(&self) -> usize;
    fn get_id(&self) -> Identifier;
    fn get_table_id_field(&self) -> String;
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

#[derive(Serialize, Deserialize, Debug)]
struct SearchResult<T> {
    pub count: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IdOnly {
    pub id: usize,
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

    pub async fn list<T>(&self) -> Vec<T>     where
        T: BaserowObject + Serialize + DeserializeOwned,
    {
        let list_url = format!("{CREATE_RECORD_URL}{}/", T::get_static_table_id());
        
        let response = self
            .client
            .get(list_url)
            .send()
            .await
            .unwrap()
            .json::<SearchResult<T>>().await.unwrap();
        
        Vec::new()
    }
    
    pub async fn create<T>(&self, obj: &T)
    where
        T: BaserowObject + Serialize,
    {
        let create_url = format!("{CREATE_RECORD_URL}{}/", obj.get_table_id().to_string());

        let request = self
            .client
            .post(create_url)
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(obj).unwrap())
            .build()
            .unwrap();

        println!("Request\n{:?}", request);
        let response = self.client.execute(request).await.unwrap();

        println!("{:?}", response);
        println!("{:?}", response.text().await.unwrap())
    }

    pub async fn update<T>(&self, obj: &T)
    where
        T: BaserowObject + Serialize,
    {
        // Need to find the rowid for the object first
        let id: String = obj.get_id().into();
        let find_url = format!(
            "{LIST_RECORD_URL}{}/?filter__{}__equal={}",
            obj.get_table_id(),
            obj.get_table_id_field(),
            id
        );

        println!("{:?}", self.client);
        let mut search_result = self
            .client
            .get(find_url)
            .send()
            .await
            .unwrap()
            .json::<SearchResult<IdOnly>>()
            .await
            .unwrap();

        if  !search_result.count.eq(&1) {
            panic!("Should only have found one object for primary id!");
        }

        let id: String = search_result.results.first().unwrap().id.to_string();

        let update_url = format!("{CREATE_RECORD_URL}{}/{}/", obj.get_table_id().to_string(), id);
        println!("{}", update_url);

        println!("{:?}", self.client);
        let response = self
            .client
            .patch(update_url)
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(obj).unwrap())
            .send()
            .await
            .unwrap();
        println!("{:?}", response);
        println!("{:?}", response.text().await.unwrap());
    }

    pub async fn generate_structs(&self, database: usize) {
        let tablelist = self.list_tables().await;

        println!("use serde::{{Deserialize, Serialize}};\n");
        println!("use crate::baserow::client::{{BaserowObject, Identifier}};\n");
        for table in tablelist {
            let struct_name = table.get_struct_name();
            let mut primary_field: Option<TableField> = None;
            println!("#[derive(Serialize, Deserialize, Debug)]");
            println!("pub struct {} {{", struct_name);
            if let Some(fields) = table.fields {
                let mut filtered_primary_fields = fields
                    .iter()
                    .filter(|field| field.is_primary())
                    .map(|field| field.clone())
                    .collect::<Vec<TableField>>();
                if filtered_primary_fields.len() != 1 {
                    panic!("got more or less than one primary field!");
                }
                primary_field = filtered_primary_fields.pop();

                for field in fields {
                    println!("#[serde(rename = \"field_{}\")]", field.get_id());
                    println!(
                        "  pub {}: Option<{}>,",
                        field.get_name().to_case(Case::Snake),
                        field.get_rust_type()
                    );
                }
            }
            println!("}}\n\n");
            println!("impl BaserowObject for {} {{\n", struct_name);
            println!("  fn get_table_id(&self) -> usize {{");
            println!("    Self::get_static_table_id()");
            println!("  }}");

            println!("  fn get_static_table_id() -> usize {{");
            println!("    {}", table.id);
            println!("  }}");

            println!("  fn get_id(&self) -> Identifier {{");
            let primary_field = primary_field.unwrap();
            Identifier::Text {
                id: primary_field.get_name(),
            };
            if primary_field.get_rust_type().eq("String") {
                println!(
                    "    Identifier::Text {{ id: self.{}.to_string() }}",
                    primary_field.get_name()
                );
            } else {
                println!(
                    "    Identifier::Numeric {{ id: self.{} }}",
                    primary_field.get_name()
                );
            }
            println!("  }}");
            println!("  fn get_table_id_field(&self) -> String {{");
            println!("    \"field_{}\".to_string()", primary_field.get_id());
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
    use std::fs;
    use crate::baserow::client::{Client, SearchResult};
    use crate::baserow::field_types::TableField;
    use crate::baserow::generated::offers;

    #[tokio::test]
    async fn generate_code() {
        let client = Client::new("");
        client.generate_structs(217366).await;
    }
    
    #[tokio::test]
    async fn test_create_offer() {
        let client = Client::new("");
        let offer = offers {
            easybill_id: 123,
            customer: "testcustomer1234".to_string(),
            amount: 12000,
            status: "draft".to_string(),
        };

        client.create(&offer).await;
    }

    #[tokio::test]
    async fn test_update_offer() {
        let client = Client::new("");
        let offer = offers {
            easybill_id: 123,
            customer: "changedcustomername".to_string(),
            amount: 12000,
            status: "sent".to_string(),
        };

        client.update(&offer).await;
    }

    #[tokio::test]
    async fn test_list_offer() {
        let client = Client::new("");
        let offers: Vec<offers> = client.list().await;
        println!("{:?}", offers);
    }

    #[test]
    fn test_deserialize_offerlist() {
        let contents = fs::read_to_string("testdata/list_offers1.json")
            .expect("Should have been able to read the file");

        let json: SearchResult<offers> =
            serde_json::from_str(&contents).expect("file should be proper JSON");

    }
    
}
