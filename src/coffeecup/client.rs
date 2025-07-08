use std::str::FromStr;
use std::sync::LazyLock;
use http::header::{ACCEPT, CONTENT_TYPE};
use reqwest::{Client as ReqwestClient, Url};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use snafu::{ResultExt, Snafu};
use crate::coffeecup::structs::TimeEntryWrapper;

static TIMEENTRY_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::from_str("https://api.coffeecupapp.com/v1/timeentries").unwrap()
});

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Invalid header value specified [{value}]:  {source}"))]
    Header {
        source: http::header::InvalidHeaderValue,
        value: String,
    },
    #[snafu(display("Failed to {msg}:  {source}"))]
    Reqwest { source: reqwest::Error, msg: String },
    #[snafu(display("Failed to serialize {msg}: {source})"))]
    SerializeRequest {
        source: serde_json::Error,
        msg: String,
    },
    #[snafu(display("Failed to {msg}:  {source}"))]
    ReqwestWithUrl {
        source: reqwest::Error,
        msg: String,
        url: String,
    },
}

pub struct CoffeeCup {
    client: reqwest::Client,
    
}

impl CoffeeCup {
    /*pub fn new(username: &str, password: &str) -> Result<Self, Error> {
        let client = BasicClient::new(ClientId::new("ttt_importer".to_string()))
            .set_client_secret(ClientSecret::new("ttt_importer".to_string()))
            .set_token_uri(TokenUrl::new("https://stackable.coffeecup.app/oauth2/token?companyurl=https://stackable.coffeecup.app".to_string()).unwrap());

        let http_client = reqwest::blocking::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let token_result = client
            .exchange_client_credentials()
            .add_scope(Scope::new("".to_string()))
            .request(&http_client).unwrap();


        Ok(Self {

        })
    }

     */
    pub fn new(token: &str) -> Result<Self, Error> {
        // Build default headers to be included with every request later on
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))
                .context(HeaderSnafu { value: token })?,
        );
        // This really shouldn't fail, but since we have the error variant .. why not?
        default_headers.insert(
            ACCEPT,
            HeaderValue::from_str("application/json").context(HeaderSnafu {
                value: "application/json",
            })?,
        );

        Ok(Self {
            client: ReqwestClient::builder()
                .default_headers(default_headers)
                .build()
                .context(ReqwestSnafu {
                    msg: "build client",
                })?,
        })
    }
    
    pub async fn create_timeentry(&self, entry: &TimeEntryWrapper) -> Result<(), Error>{

        let request = self
            .client
            .post(TIMEENTRY_URL.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&entry).context(SerializeRequestSnafu {
                msg: format!("{:?}", entry),
            })?)
            .build()
            .context(ReqwestSnafu {
                msg: "build create request",
            })?;

        println!("Request\n{:?}", request);
        let response = self
            .client
            .execute(request)
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send create request",
                url: TIMEENTRY_URL.as_ref(),
            })?;

        println!("{:?}", response);
        Ok(())
    }
}


/*
curl --request GET \
  --url https://api.coffeecupapp.com/v1/timeentries \
  --header 'Authorization: Bearer 2b80e7b35cf11fff10f65020f27e8e215f093a39' \
  --header 'User-Agent: insomnia/11.2.0'
 */