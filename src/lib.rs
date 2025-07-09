pub mod easybill;
pub mod generated;
pub mod bdwh;
pub mod coffeecup;

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use baserow_client::client::Client;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct SearchResult<T> {
        pub count: usize,
        pub next: Option<String>,
        pub previous: Option<String>,
        pub results: Vec<T>,
    }






}