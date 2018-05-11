extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_s3;
extern crate tokio_core;

use rusoto_core::reactor::RequestDispatcher;
use rusoto_core::Region;
use rusoto_credential::StaticProvider;
use rusoto_s3::{S3, S3Client};

fn main() {
    env_logger::init();

    let region = Region::Custom {
        name: "unused".to_owned(),
        endpoint: "http://localhost:9000".to_owned(),
    };
    // let region = Region::UsWest2;
    let access_key = "ANTN35UAENTS5UIAEATD".to_owned();
    let secret_key = "TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur".to_owned();
    let cred_provider = StaticProvider::new_minimal(access_key, secret_key);
    let client = S3Client::new(RequestDispatcher::default(), cred_provider, region);
    client.list_buckets().sync().unwrap();
}
