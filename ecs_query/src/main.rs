extern crate rusoto_core;
extern crate rusoto_ecs;

use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_ecs::{Ecs, EcsClient, ListClustersRequest};
use std::time::{Duration, Instant};
use rusoto_core::credential::ChainProvider;
use std::process;

fn main() {
    let mut chain = ChainProvider::new();
    chain.set_timeout(Duration::from_millis(200));
    let ecs_client = EcsClient::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        chain,
        Region::UsEast1,
    );

    

    let cluster_request = ListClustersRequest { 
        max_results: Some(10),
        next_token: None
    };

    let start = Instant::now();

    let clusters = match ecs_client.list_clusters(cluster_request).sync() {
        Err(e) => { 
            println!("Error: {}", e); 
            process::exit(1);
        },
        Ok(clusters) => clusters,
    };

    println!("Finished in {:?}", Instant::now().duration_since(start));


    for arn in clusters.cluster_arns.unwrap().iter() {
        
        println!("{}", split_last_string(&arn) );
    }
    
    
}

fn split_last_string(s: &String) -> String {
    match s.rsplit("/").next() {
        None => String::new(),
        Some(elem) => String::from(elem),
    }
}