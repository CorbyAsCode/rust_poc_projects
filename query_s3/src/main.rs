extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::credential::ChainProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_s3::{S3, S3Client};
use std::time::{Duration, Instant};
use std::process;

fn main() {
    let mut chain = ChainProvider::new();
    chain.set_timeout(Duration::from_millis(200));
    let s3client = S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        chain,
        Region::UsEast1,
    );

    let start = Instant::now();
    //println!("Starting up at {:?}", start);

    let buckets_found = match s3client.list_buckets().sync() {
        Err(err) => { 
            println!("{:?}", err);
            process::exit(1);
        }
        Ok(buckets) => buckets,
    };
    println!("Query took {:?}", Instant::now().duration_since(start));

    let start = Instant::now();
    for bucket in buckets_found.buckets.unwrap().iter() {
        match &bucket.name {
            Some(name) => println!("{}", name),
            None => ()
        }
        //println!("{:?}", bucket);
    }
    println!("Matching took {:?}", Instant::now().duration_since(start));
}
