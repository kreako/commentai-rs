#[macro_use]
extern crate diesel;

mod models;
mod schema;

use crate::models::InsertComment;
use crate::schema::comments;
use commentai_rs_data::Comment;
use diesel::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Diesel error: {0:?}")]
    Diesel(#[from] diesel::result::Error),
    #[error("Missing author_ip field on comment")]
    MissingAuthorIp,
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
