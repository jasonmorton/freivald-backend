#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
// use crate::rocket::tokio::io::{AsyncRead, AsyncReadExt};
// use rocket::response::status::NotFound;
// use rocket::tokio::fs::File;
// use rocket_contrib::serve::StaticFiles;
//use tokio::io::{self, AsyncReadExt};
use rocket::fs::{relative, FileServer};

use ark_bls12_381::Fr as F;
use ark_ff::BigInteger256;
use nalgebra::DMatrix;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

pub fn to_bytes(m: DMatrix<F>) -> DMatrix<[u64; 4]> {
    m.map(|alpha| alpha.0 .0)
}
pub fn from_bytes(m: DMatrix<[u64; 4]>) -> DMatrix<F> {
    m.map(|bytes| F::from(ark_ff::BigInteger256(bytes)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub a: DMatrix<[u64; 4]>,
    pub b: DMatrix<[u64; 4]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub c: DMatrix<[u64; 4]>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// this literally gives the file, like source, not serves it
// #[get("/<staticfile>")]
// async fn retrieve(staticfile: &str) -> Option<File> {
//     let filename = format!("static/{staticfile}", staticfile = staticfile);
//     File::open(&filename).await.ok()
// }

// #[get("/<staticfile>")]
// async fn retrieve(staticfile: &str) -> String {
//     let filename = format!("static/{staticfile}", staticfile = staticfile);
//     let mut file = File::open(&filename).await.unwrap();
//     let mut buffer = String::new();
//     file.read_to_string(&mut buffer).await.unwrap();
//     buffer
// }

// deserialize, multiply, reserialize
//#[post("/multiply", format = "json", data = "<json_request>")] // format = "json" ignores unless it is application/json
#[post("/", data = "<json_request>")]
pub fn multiply(json_request: Json<Request>) -> Json<Response> {
    println!("{:?}", json_request);
    let a = from_bytes(json_request.a.clone());
    let b = from_bytes(json_request.b.clone());
    let c = a * b;
    let c64 = to_bytes(c);
    let resp = Response { c: c64 };
    resp.into()
}

#[launch()]
fn rocket() -> _ {
    rocket::build()
        .mount("/mul", routes![multiply])
        .mount("/", FileServer::from(relative!("static")))
    //        .mount("/", routes![retrieve])
    //        .mount("/", StaticFiles::from("./static"))
}

// fn main() {
//     rocket::ignite()
//         .mount("/mul", routes![index])
//         .mount("/", StaticFiles::from("./static"))
//         .launch();
// }
