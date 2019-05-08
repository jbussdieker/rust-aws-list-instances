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

fn handle_output(output: rusoto_ec2::DescribeInstancesResult) {
    match output.reservations {
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
        Ok(output) => {
            handle_output(output)
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
