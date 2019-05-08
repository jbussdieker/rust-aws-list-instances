use rusoto_core::Region::UsEast1;
use rusoto_ec2::{Ec2, Ec2Client, Instance, Reservation, DescribeInstancesRequest, DescribeInstancesResult};
use std::vec::Vec;

fn handle_instances(instance_list: Vec<Instance>) {
    for instance in instance_list {
        println!("Instance ID: {}", instance.instance_id.expect("Missing instance ID"))
    }
}

fn handle_reservations(reservation_list: Vec<Reservation>) {
    for reservation in reservation_list {
        match reservation.instances {
            Some(instance_list) => {
                handle_instances(instance_list)
            },
            None => println!("No instances found!")
        }
    }
}

fn handle_result(result: DescribeInstancesResult) {
    match result.reservations {
        Some(reservation_list) => {
            handle_reservations(reservation_list)
        },
        None => println!("No reservations found!")
    }
}

fn main() {
    let client = Ec2Client::new(UsEast1);
    let describe_instances_request: DescribeInstancesRequest = Default::default();

    match client.describe_instances(describe_instances_request).sync() {
        Ok(result) => {
            handle_result(result)
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
