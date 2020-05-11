use actix::prelude::*;
use std::net;
use structopt::StructOpt;
use tokio::io::split;
use tokio::net::TcpStream;
use tokio_util::codec::FramedRead;

use commentai_rs_actors::cli::{CliActor, CliCmd};
use commentai_rs_actors::local_codec::{AdminRequest, CliCodec};

#[derive(StructOpt, Debug)]
/// Command line tool for tamie
enum Cmd {
    /// List all known comments
    List {
        #[structopt(short, long)]
        /// Display full content of comment
        full: bool,
        /// Optional filter on comment content
        filter: Option<String>,
    },
    /// Delete a comment
    Delete {
        /// Id of the comment to delete
        id: i32,
    },
}

#[actix_rt::main]
async fn main() {
    let cmd = Cmd::from_args();

    // Connect to admin interface
    let localhost = net::IpAddr::V4(net::Ipv4Addr::LOCALHOST);
    let addr = net::SocketAddr::new(localhost, 8888);

    let stream = TcpStream::connect(&addr).await.unwrap();

    let cli = CliActor::create(|ctx| {
        let (r, w) = split(stream);
        CliActor::add_stream(FramedRead::new(r, CliCodec), ctx);
        CliActor::new(actix::io::FramedWrite::new(w, CliCodec, ctx))
    });

    println!("{:#?}", cmd);
    match cmd {
        Cmd::List { full, filter } => {
            let answer = cli.send(CliCmd(AdminRequest::List { filter: None })).await;
            answer.unwrap().unwrap();
        }
        Cmd::Delete { id } => {}
    }
}
