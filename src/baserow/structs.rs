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
}
