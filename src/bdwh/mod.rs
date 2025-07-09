pub mod structs;

use crate::bdwh::structs::TimeEntry;
use crate::bdwh::Error::ExecuteSql;
use snafu::{ResultExt, Snafu};
use std::{io, vec};
use trino_rust_client::auth::Auth;
use trino_rust_client::{Client, ClientBuilder, Row, Trino};

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Error obtaining results for  [{stmt}]: {source}"))]
    ExecuteSql {
        source: trino_rust_client::error::Error,
        stmt: String,
    },
}

pub struct Bdwh {
    client: Client,
}

impl Bdwh {
    pub fn new(username: &str, password: &str) -> Self {
        let client = ClientBuilder::new(username, "localhost")
            .port(8443)
            .catalog("lakehouse")
            .schema("bdwh")
            .auth(Auth::new_basic(username, Some(password)))
            .secure(true)
            .no_verify(true)
            .build()
            .unwrap();
        Bdwh { client }
    }

    pub async fn get_entries_for_ban(&self, ban: &str) -> Result<Vec<TimeEntry>, Error> {
        let sql = format!(
            "select * from lakehouse.bdwh.project_migration where ban = '{}'",
            ban
        );

        Ok(self
            .client
            .get_all::<TimeEntry>(sql.clone().into())
            .await
            .context(ExecuteSqlSnafu { stmt: sql })?
            .into_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::bdwh::Bdwh;
    use baserow_client::client::Client;
    use crate::coffeecup::client::CoffeeCup;
    use crate::coffeecup::structs::TimeEntryWrapper;

    #[tokio::test]
    async fn test_get_times() {
        let client = Bdwh::new("admin", "");
        let cc_client = CoffeeCup::new("").unwrap();

        let mut test = client.get_entries_for_ban("001105").await.unwrap();

        for entry in test {
            let cc_entry: crate::coffeecup::structs::TimeEntryWrapper = entry.into();
            println!("{}", serde_json::to_string(&cc_entry).unwrap());
            //cc_client.create_timeentry(&cc_entry).await.unwrap();
            println!("Successfully created entry: {}: {}", cc_entry.timeEntry.day, cc_entry.timeEntry.duration)
        }
    }
}