use crate::local_codec::CliCodec;
use crate::local_codec::{AdminRequest, AdminResponse};
use actix::prelude::*;
use std::io;
use std::sync::Arc;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio::sync::Notify;

pub struct CliActor {
    tcp: actix::io::FramedWrite<WriteHalf<TcpStream>, CliCodec>,
    response_notify: Arc<Notify>,
}

impl CliActor {
    pub fn new(tcp: actix::io::FramedWrite<WriteHalf<TcpStream>, CliCodec>) -> Self {
        CliActor {
            tcp: tcp,
            response_notify: Arc::new(Notify::new()),
        }
    }
}

macro_rules! wait_response {
    ($self:expr) => {{
        let notify = $self.response_notify.clone();
        Box::pin(async move {
            notify.notified().await;
            Ok(())
        })
    }};
}

/// This a standard actor
impl Actor for CliActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct CliCmd(pub AdminRequest);

/// Handle stdin commands
impl Handler<CliCmd> for CliActor {
    type Result = ResponseFuture<Result<(), ()>>;

    fn handle(&mut self, msg: CliCmd, _: &mut Context<Self>) -> Self::Result {
        self.tcp.write(msg.0);
        // Here just wait for the notify of the StreamHandler<AdminResponse>::handle
        // the handler will deal with printing
        return wait_response!(self);
    }
}

impl actix::io::WriteHandler<io::Error> for CliActor {}

impl StreamHandler<Result<AdminResponse, io::Error>> for CliActor {
    fn handle(&mut self, msg: Result<AdminResponse, io::Error>, ctx: &mut Context<Self>) {
        match msg {
            Ok(AdminResponse::List { filter, comments }) => {
                println!("List of comments");
                for comment in comments {
                    println!(
                        "{: >5} {} {: <10} {: <30}",
                        comment.id.unwrap(),
                        comment.dt.to_rfc3339(),
                        comment.author_name.unwrap_or(String::new()),
                        comment.title.unwrap_or(String::new()),
                    );
                }
                self.response_notify.notify();
            }
            Ok(AdminResponse::Delete { id }) => {
                println!("Received delete with id {}", id);
            }
            // In case error Let's stop myself
            // not the most elegant thing, but for now it will do
            _ => ctx.stop(),
        }
    }
}
