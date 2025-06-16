pub mod easybill;
pub mod generated;

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use baserow_client::client::Client;
    use serde::{Deserialize, Serialize};
    use crate::generated::customersuccess::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct SearchResult<T> {
        pub count: usize,
        pub next: Option<String>,
        pub previous: Option<String>,
        pub results: Vec<T>,
    }

    #[tokio::test]
    async fn test_list_companies() {
        let client = Client::new(&std::env::var("BASEROW_TOKEN").unwrap(), None).unwrap();
        
        match client.list::<Companies>().await {
            Ok(result) => {
                println!("Got {} companies", result.len());
                for schedule in &result {
                    println!("{:?}", schedule);
                }
            },
            Err(e) => {assert!(false)},
        }
    }

    #[tokio::test]
    async fn test_list_jira() {
        let client = Client::new(&std::env::var("BASEROW_TOKEN").unwrap(), None).unwrap();

        match client.list::<Companies>().await {
            Ok(result) => {
                println!("Got {} companies", result.len());
                for schedule in &result {
                    println!("{:?}", schedule);
                }
            },
            Err(e) => {assert!(false)},
        }
    }

    #[test]
    fn test_cust() {
        let content = read_to_string("testdata/list_customers_response.json").unwrap();
        println!("{}", content);
        println!("{:?}", serde_json::from_str::<SearchResult<Companies>>(&content));
    }
}
