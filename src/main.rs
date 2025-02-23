mod data;

use anyhow::Context;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use std::fs;
use std::env;

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
    let config = Config::generate()?;
    let args = Arguments::parse();
    let settings = Settings::combine(args, config)?;

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((settings.host_ip.ip, if let Some(x) = settings.host_ip.port { x } else {8080u16}))?
        .run()
        .await?;

    Ok(())
}
