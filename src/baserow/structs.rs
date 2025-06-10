use serde::{Deserialize, Serialize};
use crate::easybill::structs::Document;

#[derive(Serialize, Deserialize, Debug)]
pub struct BaserowOffer {
    easybill_id: usize,
    customer: String,
    amount: String,
    status: isize,
}

impl From<crate::easybill::structs::Document> for BaserowOffer {
    fn from(value: Document) -> Self {
        BaserowOffer{
            easybill_id: value.id,
            customer: value.customer_snapshot.company_name,
            amount: value.amount.to_string(),
            status: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
    use crate::baserow::structs::BaserowOffer;

    #[tokio::test]
    async fn test_create() {
        let offer = BaserowOffer{
            easybill_id: 1234,
            customer: "test".to_string(),
            amount: "0.0".to_string(),
            status: 1,
        };
        let request_url = "https://api.baserow.io/api/database/rows/table/568215/?user_field_names=true";

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.append(AUTHORIZATION, HeaderValue::from_str("Token Due3bnmHOu4qujhru0qYTTaUd3BmlDhU").unwrap());
        let response = client
            .post(request_url)
            .body(serde_json::to_string(&offer).unwrap())
            .headers(headers)
            .send()
            .await.unwrap();

        println!("{:?}", response);
    }

}
