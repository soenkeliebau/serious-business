use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    customer_snapshot: Customer,
    is_draft: bool,
    id: usize,
    #[serde(rename(deserialize = "type"))]
    doctype: String,
    status: Option<String>,
    login_id: usize,
    created_at: String,
    edited_at: String,
    amount: isize,
    amount_net: isize,
    currency: String,
    attachment_ids: Vec<isize>,
    items: Vec<DocumentItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    company_name: String,
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
    use crate::structs::Document;

    #[test]
    fn test_document() {
        let file = File::open("examples/document_webhook.json").expect("Failed to read test data from file!");
        let reader = BufReader::new(file);

        let document: Document = serde_json::from_reader(reader).expect("Failed to parse as document!");

        assert_eq!(document.doctype, "OFFER");
    }

    }
