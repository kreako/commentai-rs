use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Deserialize, Serialize)]
/// A struct representing a comment
pub struct Comment {
    /// Id of the comment - optional
    pub id: Option<i32>,
    /// Title of the comment - optional
    pub title: Option<String>,
    /// Content of the comment
    pub content: String,

    /// Author name - optional
    pub author_name: Option<String>,
    /// Author email - optional
    pub author_email: Option<String>,
    /// Author ip
    pub author_ip: IpAddr,

    /// Datetime of the comment in utc
    // easier to not mess up in utc
    pub dt: DateTime<Utc>,

    /// Url to comment
    pub url: String,
}
