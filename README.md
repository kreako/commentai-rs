# commentai-rs

## What is it ?

commentai-rs is a comment hosting service geared toward statically generated site (think [zola](https://www.getzola.org/) for example) and command line addicts.

For now, a single HTTP Post endpoint is provided `/new-comment` to post a comment.

Every admin activities is done through a command line client, directly on the server hosting the service.

## Status ?

Very experimental, and not deployed anywhere (yet).

## Techno ?

Built in rust (using async) on top of tokio, actix, actix-web and sqlite.

## JSON model

To post a new comment following fields are accepted :

* title : a string - optional
* content : a string - mandatory
* author_name : a string - optional
* author_email : a string - optional
* url (the page on which the comment is posted) : a string - mandatory

By itself, the system generates :

* id : i32
* author_ip : `std::net::IpAddr`
* dt (date and time of the post) : `chrono::DateTime`
