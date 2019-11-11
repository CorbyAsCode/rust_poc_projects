use rusoto_core::credential::ChainProvider;
#[allow(dead_code)]
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_ecs::{
    DescribeServicesRequest, Ecs, EcsClient, ListServicesRequest, ListTasksRequest, Service,
};
use std::collections::HashMap;
use std::process;
use std::time::{Duration, Instant};

fn main() {
    let mut chain = ChainProvider::new();
    chain.set_timeout(Duration::from_millis(200));
    let ecs_client = EcsClient::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        chain,
        Region::UsEast1,
    );

    get_services_with_tasks(&ecs_client);
}

fn split_last_string(s: &String) -> String {
    match s.rsplit("/").next() {
        None => String::new(),
        Some(elem) => String::from(elem),
    }
}

/*
fn get_images_of_task_definition<'a>(
    ecs_client: &'a EcsClient,
    task_definitions: Vec<String>,
) //-> impl Future<Item = Vec<String>, Error = Error> + 'a {
    {
    let get_images_futures = task_definitions.into_iter().map(move |td| {
        let task_definition_req = DescribeTaskDefinitionRequest {
            task_definition: td,
            include: Some(Vec::new()),
        };
        ecs_client
            .describe_task_definition(task_definition_req)
            .map(|task_definition_res| {
                task_definition_res
                    .task_definition
                    .and_then(|td| td.container_definitions)
                    .and_then(|cds| cds.last().cloned())
                    .and_then(|cd| cd.image)
                    .and_then(|image| Some(image))
            })
    });

    join_all(get_images_futures)
        .map(|found_images| found_images.into_iter().flatten().collect())
        .map_err(|err| err.into())
}
*/

fn get_all_services(ecs_client: &EcsClient, cluster: &String) -> Vec<String> {
    let service_request = ListServicesRequest {
        cluster: Some(cluster.clone()),
        max_results: Some(100),
        ..Default::default()
    };

    match ecs_client.list_services(service_request).sync() {
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
        Ok(services) => services
            .service_arns
            .unwrap()
            .iter()
            .map(|arn| split_last_string(arn))
            .collect(),
    }
}

fn describe_services(
    ecs_client: &EcsClient,
    cluster: &String,
    service_names: Vec<String>,
) -> Vec<Service> {
    let mut describe_services: Vec<Service> = Vec::new();
    for service_name_group in service_names.chunks(10) {
        let desc_service_req = DescribeServicesRequest {
            cluster: Some(cluster.clone()),
            services: service_name_group.to_vec(),
            ..Default::default()
        };

        describe_services.extend(
            match ecs_client.describe_services(desc_service_req).sync() {
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(1);
                }
                Ok(services) => services.services.unwrap(),
            },
        )
    }
    describe_services
}

fn find_active_services(services: Vec<Service>) -> Vec<String> {
    let mut active_services: Vec<String> = Vec::new();
    for service in services {
        if service.running_count.unwrap() > 0 {
            println!(
                "Running service: {}, count: {}",
                service.service_name.clone().unwrap(),
                service.running_count.unwrap()
            );
            active_services.push(service.service_name.unwrap());
        } else {
            println!(
                "Not running service: {}, count: {}",
                service.service_name.unwrap(),
                service.running_count.unwrap()
            );
        }
    }
    active_services
}

fn map_tasks_to_services(
    ecs_client: &EcsClient,
    cluster: &String,
    services: Vec<String>,
) -> HashMap<String, Vec<String>> {
    let mut services_and_tasks: HashMap<String, Vec<String>> = HashMap::new();
    for service in services {
        let active_tasks_req = ListTasksRequest {
            cluster: Some(cluster.clone()),
            container_instance: None,
            desired_status: None,
            family: None,
            launch_type: None,
            max_results: Some(100),
            next_token: None,
            service_name: Some(String::from(&service)),
            started_by: None,
        };

        let tasks: Vec<String> = match ecs_client.list_tasks(active_tasks_req).sync() {
            Err(e) => {
                println!("Error: {}", e);
                process::exit(1);
            }
            Ok(tasks) => tasks.task_arns.unwrap(),
        };
        //task_arns.append(&mut tasks);
        services_and_tasks.insert(service, tasks);
    }
    services_and_tasks
}

fn get_services_with_tasks(ecs_client: &EcsClient) -> HashMap<String, Vec<String>> {
    //let cluster_name = String::from("PLATENG-TESTLAB");
    let cluster_name = String::from("TCE-ECS-CLUSTER-INTERNAL-02");

    let start = Instant::now();

    let all_service_names = get_all_services(ecs_client, &cluster_name);

    println!(
        "Finished all_service_names in {:?}",
        Instant::now().duration_since(start)
    );
    //println!("all service_names: {:?}, count: {}", all_service_names, all_service_names.len());

    /* Just an example of how to transform a string
    let service_names = service_arns
        .iter()
        .map(|arn| format!("{}test", arn))
        .collect::<Vec<std::string::String>>();

    println!("service_names: {:?}", service_names);
    */

    let start = Instant::now();

    let describe_services: Vec<Service> =
        describe_services(ecs_client, &cluster_name, all_service_names);

    println!(
        "Finished describe_services in {:?}",
        Instant::now().duration_since(start)
    );

    let start = Instant::now();

    let active_services = find_active_services(describe_services);

    println!(
        "Finished for service in describe_services in {:?}",
        Instant::now().duration_since(start)
    );
    println!("number of active services: {}", active_services.len());
    //println!("active services: {:?}", active_services);

    let start = Instant::now();
    let services_and_tasks = map_tasks_to_services(ecs_client, &cluster_name, active_services);

    println!(
        "Finished for service in active_services in {:?}",
        Instant::now().duration_since(start)
    );
    println!("services_and_tasks: {:?}", services_and_tasks);

    /*
    {
        # container id: {task id, service name}
        47djw92kej39: {
            task_id: f749ej3983-38o56kf9
            service_name: my_cool_stuff
        },
        g84832i2i3i7: {
            task_id: f749ej3983-38o56kf9
            service_name: my_crap_stuff
        }
    }
    */
    HashMap::new()
}
