fn handle_instances(instance_list: std::vec::Vec<rusoto_ec2::Instance>) {
    for instance in instance_list {
        println!("Instance ID: {}", instance.instance_id.expect("Missing instance ID"))
    }
}

fn handle_reservations(reservation_list: std::vec::Vec<rusoto_ec2::Reservation>) {
    for reservation in reservation_list {
        match reservation.instances {
            Some(instance_list) => {
                handle_instances(instance_list)
            },
            None => println!("No instances found!")
        }
    }
}

fn handle_result(result: rusoto_ec2::DescribeInstancesResult) {
    match result.reservations {
        Some(reservation_list) => {
            handle_reservations(reservation_list)
        },
        None => println!("No reservations found!")
    }
}

fn main() {
    let client = rusoto_ec2::Ec2Client::new(rusoto_core::Region::UsEast1);
    let describe_instances_request: rusoto_ec2::DescribeInstancesRequest = Default::default();

    match rusoto_ec2::Ec2::describe_instances(&client, describe_instances_request).sync() {
        Ok(result) => {
            handle_result(result)
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
