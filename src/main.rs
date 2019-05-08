use rusoto_core::Region::UsEast1;
use rusoto_ec2::{Ec2, Ec2Client, Instance, Reservation, DescribeInstancesRequest, DescribeInstancesResult};
use std::vec::Vec;

fn handle_instances(instances: Vec<Instance>) {
    for instance in instances {
        println!("Instance ID: {}", instance.instance_id.expect("Missing instance ID"))
    }
}

fn handle_reservations(reservations: Vec<Reservation>) {
    for reservation in reservations {
        match reservation.instances {
            Some(instances) => {
                handle_instances(instances)
            },
            None => println!("No instances found!")
        }
    }
}

fn handle_result(result: DescribeInstancesResult) {
    match result.reservations {
        Some(reservations) => {
            handle_reservations(reservations)
        },
        None => println!("No reservations found!")
    }
}

fn main() {
    let client = Ec2Client::new(UsEast1);
    let request: DescribeInstancesRequest = Default::default();

    match client.describe_instances(request).sync() {
        Ok(result) => {
            handle_result(result)
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
