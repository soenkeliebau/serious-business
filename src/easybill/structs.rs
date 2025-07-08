use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub customer_snapshot: Customer,
    pub is_draft: bool,
    pub id: usize,
    #[serde(rename(deserialize = "type"))]
    pub doctype: String,
    pub status: Option<String>,
    pub login_id: usize,
    pub created_at: String,
    pub edited_at: String,
    pub amount: isize,
    pub amount_net: isize,
    pub currency: String,
    pub attachment_ids: Vec<isize>,
    pub items: Vec<DocumentItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub id: usize,
    #[serde(rename = "display_name")]
    pub company_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentItem {
    description: String,
    item_type: String,
    number: String,
    single_price_gross: isize,
    single_price_net: isize,
    total_price_gross: isize,
    total_price_net: isize,
    total_vat: isize,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use crate::easybill::structs::Document;

    #[test]
    fn test_document() {
        let file = File::open("examples/document_webhook.json").expect("Failed to read test data from file!");
        let reader = BufReader::new(file);

        let document: Document = serde_json::from_reader(reader).expect("Failed to parse as document!");

        assert_eq!(document.doctype, "OFFER");
    }

    }
