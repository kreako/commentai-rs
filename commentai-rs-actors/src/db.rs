use actix::prelude::*;

use commentai_rs_data::Comment;
use commentai_rs_sqlite::{insert_comment, Db, Error};

pub struct DbExecutor {
    db: Db,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new(db_url: &str) -> Self {
        DbExecutor {
            db: Db::new(db_url),
        }
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct Insert(pub Comment);

impl Handler<Insert> for DbExecutor {
    type Result = <Insert as Message>::Result;

    fn handle(&mut self, msg: Insert, _: &mut Self::Context) -> Self::Result {
        insert_comment(&self.db, msg.0)
    }
}
