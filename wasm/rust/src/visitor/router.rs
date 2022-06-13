use anyhow::Result;
use http::Method;
use spin_sdk::{
    http::{Request, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::Result as Res;

#[derive(Serialize, Deserialize)]
struct VisitorEntity {
    username: String,
    name: String,
    password: String,
    phoneNumber: String,
    acceptedCommercial: bool,
    acceptedTerms: bool,
    userType: bool
}

pub fn route(req: &Request, parsed_url: Vec<&str>)-> Result<Response> {
    return match req.method() {
        &Method::POST => {
            if parsed_url[parsed_url.len() - 2] == "search" {
                return Ok(http::Response::builder()
                    .status(200)
                    .body(Some("Searching Visitor POST".into()))?)
            }
            let body = req.body().clone();
            let _body: Res<VisitorEntity> = serde_json::from_slice(&body.unwrap());
            Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("POST Visitor at {} {}", req.uri(), parsed_url[4]).into()))?)
        }
        &Method::GET => {
            Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Getting Visitor {} Id", parsed_url[parsed_url.len() - 2]).into()))?)
        }
        &Method::DELETE => {
            Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Deleting Visitor {} Id", parsed_url[parsed_url.len() - 2]).into()))?)
        }
        _ => {
            Ok(http::Response::builder()
                .status(500)
                .body(Some("".into()))?)
        }
    }
}