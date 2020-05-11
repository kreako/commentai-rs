use crate::schema::comments;
use crate::Error;
use chrono::{DateTime, Utc};
use commentai_rs_data::Comment;
use serde::Deserialize;
use std::convert::TryInto;

#[derive(Insertable, Deserialize)]
#[table_name = "comments"]
pub struct InsertComment {
    pub title: Option<String>,
    pub content: String,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub author_ip: String,
    pub dt: String,
    pub url: String,
}

impl From<Comment> for InsertComment {
    fn from(comment: Comment) -> Self {
        InsertComment {
            title: comment.title,
            content: comment.content,
            author_name: comment.author_name,
            author_email: comment.author_email,
            author_ip: comment.author_ip.to_string(),
            dt: comment.dt.to_rfc3339(),
            url: comment.url,
        }
    }
}

#[derive(Queryable, Deserialize)]
pub struct SelectComment {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: String,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub author_ip: String,
    pub dt: String,
    pub url: String,
}

impl TryInto<Comment> for SelectComment {
    type Error = Error;
    fn try_into(self) -> Result<Comment, Self::Error> {
        let author_ip = self.author_ip.parse()?;
        let dt_local = DateTime::parse_from_rfc3339(&self.dt)?;
        let dt_utc = dt_local.with_timezone(&Utc);
        Ok(Comment {
            id: self.id,
            title: self.title,
            content: self.content,
            author_name: self.author_name,
            author_email: self.author_email,
            author_ip: author_ip,
            dt: dt_utc,
            url: self.url,
        })
    }
}
