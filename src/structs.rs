use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    customer_snapshot: Customer,
    is_draft: bool,
    id: usize,
    #[serde(rename(deserialize = "type"))]
    doctype: String,
    status: String,
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
