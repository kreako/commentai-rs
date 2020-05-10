use crate::schema::comments;
use commentai_rs_data::Comment;
use serde::Deserialize;

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
            dt: comment.dt.to_rfc2822(),
            url: comment.url,
        }
    }
}
