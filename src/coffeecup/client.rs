use crate::coffeecup::structs::{Customer, Project, Tag, Task, TaskAssignment, TimeEntryWrapper, TrackingType, ValidationStatus};
use chrono::Duration;
use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse, BasicTokenType,
};
use oauth2::{
    AccessToken, AuthUrl, Client, ClientId, ClientSecret, EmptyExtraTokenFields, EndpointNotSet,
    EndpointSet, ResourceOwnerPassword, ResourceOwnerUsername, StandardRevocableToken,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client as ReqwestClient, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::str::FromStr;
use std::sync::LazyLock;

static TIMEENTRY_URL: LazyLock<Url> =
    LazyLock::new(|| Url::from_str("https://api.coffeecupapp.com/v1/timeentries").unwrap());
static TAGS_URL: LazyLock<Url> =
    LazyLock::new(|| Url::from_str("https://api.coffeecup.app/v1/tags").unwrap());
static TAGASSIGNMENTS_URL: LazyLock<Url> =
    LazyLock::new(|| Url::from_str("https://api.coffeecup.app/v1/tagassignments").unwrap());
static AUTH_URL: LazyLock<Url> =
    LazyLock::new(|| Url::from_str("https://api.coffeecup.app/v1/tagassignments").unwrap());
static TASKS_URL: LazyLock<Url> =
    LazyLock::new(|| Url::from_str("https://api.coffeecup.app/v1/tasks").unwrap());
static TASKASSIGNMENTS_URL: LazyLock<Url> =
    LazyLock::new(|| Url::from_str("https://api.coffeecup.app/v1/taskassignments").unwrap());
static PROJECTS_URL: LazyLock<Url> =
    LazyLock::new(|| Url::from_str("https://api.coffeecup.app/v1/projects").unwrap());
static CUSTOMER_URL: LazyLock<Url> =
    LazyLock::new(|| Url::from_str("https://api.coffeecup.app/v1/clients").unwrap());
static TOKEN_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::from_str(
        "https://stackable.coffeecup.app/oauth2/token?companyurl=https://stackable.coffeecup.app",
    )
    .unwrap()
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
    #[snafu(display("Found no tag with name [{name}]"))]
    NoTag { name: String },
    #[snafu(display("Found too many tags for tag name [{name}] - found {} tags"))]
    TooManyTags { name: String, amount: usize },
    #[snafu(display("Failed to parse {url_type} url: {source}"))]
    ParseUrl {
        url_type: String,
        source: oauth2::url::ParseError,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct TagAssigment {
    id: usize,
    record: usize,
    model: String,
    tag: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeEntry {
    createdAt: String,
    updatedAt: String,
    id: usize,
    trackingType: TrackingType,
    startTime: Option<String>,
    endTime: Option<String>,
    day: String,
    sorting: usize,
    duration: usize,
    durationRounded: usize,
    durationRoundedOverride: Option<usize>,
    estimate: Option<usize>,
    running: bool,
    comment: String,
    hourlyRate: f64,
    billedAt: Option<String>,
    billable: bool,
    validationStatus: ValidationStatus,
    wasRejected: bool,
    firstSubmissionTime: Option<String>,
    approvedOn: Option<String>,
    externalId: Option<String>,
    team: usize,
    task: usize,
    project: usize,
    user: usize,
    invoice: usize,
    approvedBy: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListTagAssignmentResponse {
    meta: ListResponseMeta,
    #[serde(rename = "tagAssignments")]
    tag_assignments: Vec<TagAssigment>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListTaskAssignmentResponse {
    meta: ListResponseMeta,
    #[serde(rename = "taskAssignments")]
    taskAssignments: Vec<TaskAssignment>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListTimeEntryResponse {
    meta: ListResponseMeta,
    #[serde(rename = "timeEntries")]
    time_entries: Vec<TimeEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponseMeta {
    skip: isize,
    limit: isize,
    total: isize,
    sort: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTagResponse {
    pub tags: Vec<Tag>,
    pub meta: ListResponseMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTaskResponse {
    pub tasks: Vec<Task>,
    pub meta: ListResponseMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse<T>
where
    T: ProvidesPayloadFieldName,
{
    #[serde(rename = "T::payload_field_name()")]
    pub payload: Vec<T>,
    pub meta: ListResponseMeta,
}

pub trait ProvidesPayloadFieldName {
    fn payload_field_name() -> &'static str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListProjectResponse {
    pub projects: Vec<Project>,
    pub meta: ListResponseMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCustomerResponse {
    #[serde(rename="clients")]
    pub customers: Vec<Customer>,
    pub meta: ListResponseMeta,
}

pub struct CoffeeCup {
    client: reqwest::Client,
    oauth_client: Client<
        BasicErrorResponse,
        BasicTokenResponse,
        BasicTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
    token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
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
    pub async fn new_with_password(username: &str, password: &str) -> Result<Self, Error> {
        let client = BasicClient::new(ClientId::new("client_id".to_string()))
            .set_client_secret(ClientSecret::new("client_secret".to_string()))
            .set_token_uri(TokenUrl::new(TOKEN_URL.to_string()).unwrap());

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let token_result = client
            .exchange_password(
                &ResourceOwnerUsername::new(username.to_string()),
                &ResourceOwnerPassword::new(password.to_string()),
            )
            .request_async(&http_client)
            .await
            .unwrap();
        println!("{:?}", token_result);
        Ok(Self {
            client: http_client,
            oauth_client: client,
            token: token_result,
        })
    }

    pub async fn get_token(&mut self) -> Result<String, Error> {
        if let Some(expiry) = self.token.expires_in() {
            if expiry < std::time::Duration::from_secs(60 * 60 * 24) {
                println!("Token expired, refreshing..");
                // Ticket is about to expire
                let test = self
                    .oauth_client
                    .exchange_refresh_token(self.token.refresh_token().unwrap())
                    .request_async(&self.client)
                    .await
                    .unwrap();
                self.token = test;
                println!("refresh done!")
            }
        } else {
            // not sure ....
        }
        Ok(self.token.access_token().clone().into_secret())
    }

    pub async fn new(token: &str) -> Result<Self, Error> {
        Self::new_with_password("", "").await
        /*
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
            oauth_client: BasicClient::new(ClientId::new("client_id".to_string()))
            .set_client_secret(ClientSecret::new("client_secret".to_string()))
            .set_token_uri(TokenUrl::new(TOKEN_URL.to_string()).unwrap());,
            token: (),
        })

         */
    }

    pub async fn get_my_projects(&mut self) -> Result<Vec<Project>, Error> {
        let mut projects = self.list_projects().await.unwrap();
        let tasks = self.list_tasks().await.unwrap();
        println!("Tasks: {:?}", tasks);
        let task_assignments = self.list_task_assignments().await.unwrap();


        let mut result: Vec<Project> = Vec::new();
        for mut project in projects {
            let task_ids:Vec<usize> = task_assignments
                .iter()
                .filter(|task_assignment| task_assignment.project.eq(&project.id))
                .map(|task_assignment: &TaskAssignment| task_assignment.task)
                .collect();
            println!("Task ids for project {:?}: {:?}", project.name, task_ids);
            let tasks: Vec<Task> = tasks.iter().filter(|task| task_ids.contains(&task.id)).map(|task| task.clone()).collect();
            println!("Tasks for project {:?}: {:?}", project.name, tasks);
            let mut new_project = project.clone();
            new_project.tasks = Some(tasks);
            result.push(new_project);

        }
        println!("Projects: {:?}", result);

        Ok(result)
    }

    pub async fn list_projects(&mut self) -> Result<Vec<Project>, Error> {
        let request = self
            .client
            .get(PROJECTS_URL.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&mut self.get_token().await?)
            .build()
            .context(ReqwestSnafu {
                msg: "build list projects request",
            })?;

        let response = self
            .client
            .execute(request)
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send list projects request",
                url: PROJECTS_URL.as_ref(),
            })?;
        println!("{:?}", response);
        Ok(response
            .json::<ListProjectResponse>()
            .await
            .unwrap()
            .projects)
    }

    pub async fn list_customer(&mut self) -> Result<Vec<Customer>, Error> {
        let request = self
            .client
            .get(CUSTOMER_URL.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&mut self.get_token().await?)
            .build()
            .context(ReqwestSnafu {
                msg: "build list customer request",
            })?;

        let response = self
            .client
            .execute(request)
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send list customer request",
                url: CUSTOMER_URL.as_ref(),
            })?;
        println!("{:?}", response);
        Ok(response
            .json::<ListCustomerResponse>()
            .await
            .unwrap()
            .customers)
    }
    async fn list_tasks(&mut self) -> Result<Vec<Task>, Error> {
        let request = self
            .client
            .get(TASKS_URL.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&mut self.get_token().await?)
            .build()
            .context(ReqwestSnafu {
                msg: "build list tasks request",
            })?;

        let response = self
            .client
            .execute(request)
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send list tasks request",
                url: TIMEENTRY_URL.as_ref(),
            })?
            .json::<ListTaskResponse>()
            .await
            .unwrap();
        Ok(response.tasks)
    }

    async fn list_task_assignments(&mut self) -> Result<Vec<TaskAssignment>, Error> {
        let request = self
            .client
            .get(TASKASSIGNMENTS_URL.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&mut self.get_token().await?)
            .build()
            .context(ReqwestSnafu {
                msg: "build list task assignments request",
            })?;

        let response = self
            .client
            .execute(request)
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send list task assignments request",
                url: TASKASSIGNMENTS_URL.as_ref(),
            })?
            .json::<ListTaskAssignmentResponse>()
            .await
            .unwrap();
        Ok(response.taskAssignments)
    }

    async fn get_tag_assigments(
        &self,
        tag_id: usize,
        model: &str,
    ) -> Result<Vec<TagAssigment>, Error> {
        let request = self
            .client
            .get(TAGASSIGNMENTS_URL.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .build()
            .context(ReqwestSnafu {
                msg: "build list tag assignments request",
            })?;

        let response = self
            .client
            .execute(request)
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send create request",
                url: TIMEENTRY_URL.as_ref(),
            })?
            .json::<ListTagAssignmentResponse>()
            .await
            .unwrap();

        Ok(response
            .tag_assignments
            .into_iter()
            .filter(|assignment| assignment.model.eq(model) && assignment.tag.eq(&tag_id))
            .collect())
    }

    pub async fn get_project_ids_by_tag(&mut self, tag: &str) -> Result<Vec<usize>, Error> {
        let tag_id = self.find_tag_by_name(tag).await?;

        let projects = self.get_tag_assigments(tag_id, "project").await.unwrap();
        println!("Found projects: {:?}", projects);
        Ok(projects
            .iter()
            .map(|assignment| assignment.record)
            .collect())
    }

    pub async fn get_timeentries(
        &self,
        team: usize,
        projects: Option<Vec<usize>>,
    ) -> Result<Vec<TimeEntry>, Error> {
        Ok(Vec::new())
    }

    async fn find_tag_by_name(&mut self, tag_name: &str) -> Result<usize, Error> {
        let request = self
            .client
            .get(TAGS_URL.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&self.get_token().await.unwrap())
            .build()
            .context(ReqwestSnafu {
                msg: "build create request",
            })?;

        let response = self
            .client
            .execute(request)
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send create request",
                url: TIMEENTRY_URL.as_ref(),
            })?
            .json::<ListTagResponse>()
            .await
            .unwrap();

        // Find tag matching the name we are looking for
        let mut matching_tags: Vec<&Tag> = response
            .tags
            .iter()
            .filter(|tag| tag.label.eq(tag_name))
            .collect();
        if let Some(found_tag) = matching_tags.pop() {
            if matching_tags.is_empty() {
                // We took the only tag that was found - success!
                Ok(found_tag.id)
            } else {
                // We found more than one tag, not good!
                Err(Error::TooManyTags {
                    name: tag_name.to_string(),
                    amount: matching_tags.len() + 1,
                })
            }
        } else {
            // We found no tags, also not good!
            Err(Error::NoTag {
                name: tag_name.to_string(),
            })
        }
    }

    pub async fn create_timeentry(&self, entry: &TimeEntryWrapper) -> Result<(), Error> {
        let request = self
            .client
            .post(TIMEENTRY_URL.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .body(
                serde_json::to_string(&entry).context(SerializeRequestSnafu {
                    msg: format!("{:?}", entry),
                })?,
            )
            .build()
            .context(ReqwestSnafu {
                msg: "build create request",
            })?;

        let response = self
            .client
            .execute(request)
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send create request",
                url: TIMEENTRY_URL.as_ref(),
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::bdwh::Bdwh;
    use crate::coffeecup::client::CoffeeCup;
    use crate::coffeecup::structs::TimeEntryWrapper;
    use baserow_client::client::Client;

    #[tokio::test]
    async fn test_get_tag() {
        let mut cc_client = CoffeeCup::new("519834cd47c02c47e16c14f1ca0522f643956d95")
            .await
            .unwrap();

        assert_eq!(cc_client.find_tag_by_name("de").await.unwrap(), 5502);
    }

    #[tokio::test]
    async fn test_get_project_by_tag() {
        let mut cc_client = CoffeeCup::new("519834cd47c02c47e16c14f1ca0522f643956d95")
            .await
            .unwrap();

        let tag_id = cc_client.find_tag_by_name("de").await.unwrap();

        //let test = cc_client.get_project_by_tag("de").await.unwrap();
    }

    #[tokio::test]
    async fn test_async() {
        let mut cc_client = CoffeeCup::new_with_password(
            "soenke.liebau@stackable.tech",
            "Rift-Safari7-Untidy-Setup-Subpanel-Lifting",
        )
        .await
        .unwrap();

        println!("test");
        let tag_id = cc_client.find_tag_by_name("de").await.unwrap();
        println!("{:?}", tag_id)
    }
}
