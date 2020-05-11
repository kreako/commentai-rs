#[macro_use]
extern crate diesel;

mod models;
mod schema;

use crate::models::{InsertComment, SelectComment};
use crate::schema::comments;
use crate::schema::comments::dsl::*;
use commentai_rs_data::Comment;
use diesel::prelude::*;
use std::convert::TryInto;
use std::net::AddrParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Diesel error : {0:?}")]
    Diesel(#[from] diesel::result::Error),
    #[error("AddrParseError : {0:?}")]
    InvalidAuthorIp(#[from] AddrParseError),
    #[error("chrono::format::ParseError : {0:?}")]
    InvalidDateTime(#[from] chrono::format::ParseError),
}

pub struct Db(SqliteConnection);

impl Db {
    pub fn new(db_path: &str) -> Self {
        Db(SqliteConnection::establish(db_path)
            .expect("Unable to establish a connection to the database :("))
    }
}

pub fn insert_comment(db: &Db, comment: Comment) -> Result<(), Error> {
    let insert_comment: InsertComment = comment.into();
    diesel::insert_into(comments::table)
        .values(&insert_comment)
        .execute(&db.0)
        .map(|_| ())
        .map_err(From::from)
}

pub fn select_comment(db: &Db, filter: Option<String>) -> Result<Vec<Comment>, Error> {
    let query: Vec<SelectComment> = comments.load::<SelectComment>(&db.0)?;
    query.into_iter().map(|c| c.try_into()).collect()
}
