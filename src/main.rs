mod data;

use anyhow::Context;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder, middleware::Logger,
http::{
    header::{self, ContentType},
    Method, StatusCode
}
};
use clap::Parser;
use std::fs;

use data::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {

    // read config file
    let config = Config::generate()?;

    // read from command line
    let args = Arguments::parse();

    //combine them together to form a Settings struct
    let settings = Settings::combine(args, config)?;

    HttpServer::new(|| {
        App::new()
            .service(Files::)
            .route("/hey", web::get().to(manual_hello))
                })
    .bind((settings.host_ip.ip, if let Some(x) = settings.host_ip.port { x } else {8080u16}))?
        .run()
        .await?;

    Ok(())
}
