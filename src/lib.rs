pub mod easybill;
pub mod generated;


#[cfg(test)]
mod tests {
    use baserow_client::client::Client;
    use crate::generated::support::Schedule;

    #[tokio::test]
    async fn test_get_schedule() {
        let client = Client::new("", None);
        
        let test: Vec<Schedule> = client.list().await;
    }
}
