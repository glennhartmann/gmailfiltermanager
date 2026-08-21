#![allow(dead_code)]

use bon::Builder;
use serde::{Deserialize, Serialize};

/// Base API endpoint URL.
const BASE_URL: &str = "https://gmail.googleapis.com/gmail/v1/users/me/settings/filters";

/// Gmail filter. See
/// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters#resource:-filter>.
#[derive(Builder, Serialize, Deserialize)]
pub struct Filter {
    /// Gmail's internal identifier for the filter.
    pub id: Option<String>,

    /// The conditions that need to be met for the filter to activate.
    pub criteria: Criteria,

    /// Action the filter performs if the criteria are met.
    pub action: Action,
}

/// The conditions that need to be met for a filter to activate. See
/// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters#criteria>.
#[derive(Clone, Builder, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Criteria {
    /// Email sender's display name or email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// Email receiver's display name or email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// Email subject contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Gmail search query. Follows syntax at <https://support.google.com/mail/answer/7190>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Like `query`, but negated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negated_query: Option<String>,

    /// Whether or not the email contains an attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachment: Option<bool>,

    /// Whether or not to include chats in the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_chats: Option<bool>,

    /// Email size in bytes. Used in conjunction with `size_comparison`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,

    /// How the `size` should be used to compare with a given email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_comparison: Option<SizeComparison>,
}

/// How the `size` should be used to compare with a given email. See
/// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters#sizecomparison>.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SizeComparison {
    /// Unspecified should probably not be used and will probably cause errors. It's undocumented.
    Unspecified,

    /// Matches messages that are smaller than `size`.
    Smaller,

    /// Matches messages that are larger than `size`.
    Larger,
}

/// Action a filter performs if the criteria are met. See
/// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters#action>.
#[derive(Clone, Builder, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    /// Label IDs to add to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_label_ids: Option<Vec<String>>,

    /// Label IDs to remove from the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_label_ids: Option<Vec<String>>,

    /// Email address to forward the message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<String>,
}

/// List of filters, as returned by the `list` API.
#[derive(Builder, Serialize, Deserialize)]
pub struct List {
    /// List of filters.
    filter: Vec<Filter>,
}

/// API client used to share a `reqwest` client between multiple API calls.
pub struct Client {
    /// `reqwest` client shared between multiple API calls.
    reqwest_client: reqwest::Client,

    /// OAuth2 access token.
    token: String,
}

impl Client {
    /// Creates a new `Client`.
    pub fn new(token: &str) -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
            token: token.to_string(),
        }
    }

    /// Calls the `create` API. See
    /// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters/create>.
    pub async fn create(&self, filter: Filter) -> anyhow::Result<reqwest::Response> {
        Ok(self
            .reqwest_client
            .post(BASE_URL)
            .body(serde_json::to_string(&filter)?)
            .bearer_auth(&self.token)
            .send()
            .await?)
    }

    /// Calls the `delete` API. See
    /// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters/delete>.
    pub async fn delete(&self, id: &str) -> anyhow::Result<reqwest::Response> {
        let url = make_id_url(id);
        Ok(self
            .reqwest_client
            .delete(url)
            .bearer_auth(&self.token)
            .send()
            .await?)
    }

    /// Calls the `get` API. See
    /// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters/get>.
    pub async fn get(&self, id: &str) -> anyhow::Result<reqwest::Response> {
        let url = make_id_url(id);
        Ok(self
            .reqwest_client
            .get(url)
            .bearer_auth(&self.token)
            .send()
            .await?)
    }

    /// Calls the `list` API. See
    /// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters/list>.
    pub async fn list(&self) -> anyhow::Result<List> {
        let resp = self
            .reqwest_client
            .get(BASE_URL)
            .bearer_auth(&self.token)
            .send()
            .await?;

        Ok(resp.json().await?)
    }
}

/// Calls the `create` API. See
/// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters/create>.
/// Use `Client` if you want to reuse the `reqwest` client between multiple API calls.
pub async fn create(token: &str, filter: Filter) -> anyhow::Result<reqwest::Response> {
    Client::new(token).create(filter).await
}

/// Calls the `delete` API. See
/// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters/delete>.
/// Use `Client` if you want to reuse the `reqwest` client between multiple API calls.
pub async fn delete(token: &str, id: &str) -> anyhow::Result<reqwest::Response> {
    Client::new(token).delete(id).await
}

/// Calls the `get` API. See
/// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters/get>.
/// Use `Client` if you want to reuse the `reqwest` client between multiple API calls.
pub async fn get(token: &str, id: &str) -> anyhow::Result<reqwest::Response> {
    Client::new(token).get(id).await
}

/// Calls the `list` API. See
/// <https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.settings.filters/list>.
/// Use `Client` if you want to reuse the `reqwest` client between multiple API calls.
pub async fn list(token: &str) -> anyhow::Result<List> {
    Client::new(token).list().await
}

/// Gets the API endpoint URL for a given filter `id`.
fn make_id_url(id: &str) -> String {
    format!("{}/{}", BASE_URL, id)
}
