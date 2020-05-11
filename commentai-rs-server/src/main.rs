use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use dotenv::dotenv;
use listenfd::ListenFd;
use serde::Deserialize;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use commentai_rs_actors::{db, local_tcp};
use commentai_rs_data::Comment;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug, Deserialize)]
struct NewComment {
    pub title: Option<String>,
    pub content: String,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub url: String,
}

impl NewComment {
    fn to_comment(self, ip: IpAddr) -> Comment {
        Comment {
            id: None,
            title: self.title,
            content: self.content,
            author_name: self.author_name,
            author_email: self.author_email,
            author_ip: ip,
            dt: Utc::now(),
            url: self.url,
        }
    }
}

async fn new_comment(
    req: HttpRequest,
    db: web::Data<Addr<db::DbExecutor>>,
    new_comment: web::Json<NewComment>,
) -> Result<HttpResponse, Error> {
    let new_comment = new_comment.into_inner();
    println!(
        "{}  {:?}",
        req.connection_info().host(),
        req.connection_info().remote()
    );
    let peer_ip = match req.connection_info().remote() {
        Some(socket) => match SocketAddr::from_str(socket) {
            Ok(s) => s.ip(),
            Err(_e) => {
                eprintln!("Error in SocketAddr parse");
                return Ok(HttpResponse::InternalServerError().into());
            }
        },
        None => return Ok(HttpResponse::InternalServerError().into()),
    };
    let comment: Comment = new_comment.to_comment(peer_ip);
    let res = db.send(db::Insert(comment)).await?;
    match res {
        Ok(()) => Ok(HttpResponse::Ok().body("Done")),
        Err(e) => {
            println!("Error: {:?}", e);
            Ok(HttpResponse::InternalServerError().into())
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_addr = SyncArbiter::start(2, move || db::DbExecutor::new(&db_url));

    // Start local admin interface
    local_tcp::local_server(8888, db_addr.clone());

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_addr.clone())
            .route("/", web::get().to(index))
            .route("/new-comment", web::post().to(new_comment))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
}
