use crate::db::{DbExecutor, Select};
use crate::local_codec::{AdminRequest, AdminResponse, LocalTcpCodec};
use actix::prelude::*;
use futures::StreamExt;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::io::{split, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::FramedRead;

/// Actor handling the local admin connection
pub struct LocalTcpActor {
    db: Addr<DbExecutor>,
    tcp: actix::io::FramedWrite<WriteHalf<TcpStream>, LocalTcpCodec>,
}

impl LocalTcpActor {
    fn new(
        db: Addr<DbExecutor>,
        tcp: actix::io::FramedWrite<WriteHalf<TcpStream>, LocalTcpCodec>,
    ) -> Self {
        LocalTcpActor { db: db, tcp: tcp }
    }
}

/// This a standard actor
impl Actor for LocalTcpActor {
    type Context = Context<Self>;
}

impl actix::io::WriteHandler<io::Error> for LocalTcpActor {}

impl StreamHandler<Result<AdminRequest, io::Error>> for LocalTcpActor {
    fn handle(&mut self, msg: Result<AdminRequest, io::Error>, ctx: &mut Context<Self>) {
        match msg {
            Ok(AdminRequest::List { filter }) => {
                println!("Received list with filter {:?}", filter);
                self.db
                    .send(Select {
                        filter: filter.clone(),
                    })
                    .into_actor(self)
                    .then(|res, act, _| {
                        match res {
                            Ok(r) => {
                                if let Ok(comments) = r {
                                    act.tcp.write(AdminResponse::List {
                                        filter: filter,
                                        comments: comments,
                                    });
                                } else {
                                    println!("Something is wrong too");
                                }
                            }
                            _ => println!("Something is wrong"),
                        }
                        actix::fut::ready(())
                    })
                    .wait(ctx);
            }
            Ok(AdminRequest::Delete { id }) => {
                println!("Received delete with id {}", id);
            }
            // In case error Let's stop myself
            // not the most elegant thing, but for now it will do
            _ => ctx.stop(),
        }
    }
}

pub fn local_server(port: u16, db: Addr<DbExecutor>) {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, port);

    actix_rt::spawn(async move {
        let db = db.clone();
        let mut listener = TcpListener::bind(&addr).await.unwrap();
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => {
                    let db = db.clone();
                    LocalTcpActor::create(|ctx| {
                        let (r, w) = split(stream);
                        LocalTcpActor::add_stream(FramedRead::new(r, LocalTcpCodec), ctx);
                        LocalTcpActor::new(db, actix::io::FramedWrite::new(w, LocalTcpCodec, ctx))
                    });
                }
                Err(_) => return,
            }
        }
    });
}
