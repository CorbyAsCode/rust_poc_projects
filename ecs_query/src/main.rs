extern crate rusoto_core;
extern crate rusoto_ecs;

use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_ecs::{Ecs, EcsClient};
use std::time::{Duration, Instant};

fn main() {
    let mut chain = ChainProvider::new();
    chain.set_timeout(Duration::from_millis(200));
    let ecsClient = EcsClient::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        chain,
        Region::UsEast1,
    );

    let tasks = ecsClient.ListTasksRequest
        .cluster()
}
