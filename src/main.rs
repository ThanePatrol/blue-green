#![feature(yeet_expr)]

use std::{process::exit, time::Duration};

const BLUE_PORT: u16 = 8082;
const GREEN_PORT: u16 = 8012;

struct Port {
    port_number: u16,
}

impl Port {
    fn new(port_number: u16) -> Self {
        Self { port_number }
    }
}

enum CurrentDeploy {
    Blue(Port),
    Green(Port),
}

impl CurrentDeploy {
    fn get_port(&self) -> u16 {
        match self {
            CurrentDeploy::Blue(port) => port.port_number,
            CurrentDeploy::Green(port) => port.port_number,
        }
    }

    fn is_same_color(&self, string: &str) -> bool {
        match self {
            CurrentDeploy::Blue(_) => string == "blue",
            CurrentDeploy::Green(_) => string == "green",
        }
    }
}

//   ports:
//      - "8080:8080"
//      - "8081:8081"
//      - "8082:8082"
//      - "9000:9999"
//  ingress-green:
//    image: service-proxy
//    container_name: ingress-green
//    ports:
//      - "8010:8080"
//      - "8011:8081"
//      - "8012:8082"
//      - "9010:9999"

//NB - this requires to be run as root for the iptables commands to succeed
fn main() {
    println!("Creating docker containers...");
    //start docker containers
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("cd ~/service-proxy && make start-local")
        .output()
        .expect("error running docker - is it installed and docker daemon running?");

    println!("Done!");
    let iptables_error_string = "Error running iptables, are you root?";

    println!("Listening blue deploy on port 8000");
    if let Ok(mut current_deploy) = init() {
        let mut buffer = String::new();

        println!("run curl http://localhost:8000 to see the current deploy");
        println!("Enter one of the following strings: 'blue' 'green' or 'exit'");
        // we read from stdin and change listener rules as appropriate
        loop {
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("Error reading from stdin");
            if buffer.ends_with('\n') {
                buffer.pop();
            }

            match buffer.as_str() {
                "green" => {
                    switch_ip_tables(&mut current_deploy, "green").expect(iptables_error_string);
                }
                "blue" => {
                    switch_ip_tables(&mut current_deploy, "blue").expect(iptables_error_string);
                }
                "exit" => exit(1),
                _ => println!("You entered {buffer}, Enter one of the following strings: 'blue' 'green' or 'exit'"),

            }
            buffer.clear();
        }
    } else {
        panic!("{iptables_error_string}");
    }
}

fn switch_ip_tables(
    current_deploy: &mut CurrentDeploy,
    requested_change: &str,
) -> Result<(), std::fmt::Error> {
    if current_deploy.is_same_color(requested_change) {
        return Ok(());
    }
    let delete_current_rule = format!(
        "iptables -t nat -D OUTPUT -p tcp --dport 8000 -j REDIRECT --to-port {}",
        current_deploy.get_port().to_string()
    );

    match current_deploy {
        CurrentDeploy::Blue(_) => *current_deploy = CurrentDeploy::Green(Port::new(GREEN_PORT)),
        CurrentDeploy::Green(_) => *current_deploy = CurrentDeploy::Blue(Port::new(BLUE_PORT)),
    }
    println!(
        "changed to {requested_change} with port {}",
        current_deploy.get_port()
    );

    //TODO - does this need to be 1 second
    std::thread::sleep(Duration::from_secs(1));

    let add_new_rule = format!(
        "iptables -t nat -A OUTPUT -p tcp --dport 8000 -j REDIRECT --to-port {}",
        current_deploy.get_port().to_string()
    );

    if let Err(_) = std::process::Command::new("sh")
        .arg("-c")
        .arg(delete_current_rule)
        .output()
    {
        do yeet std::fmt::Error;
    }

    if let Err(_) = std::process::Command::new("sh")
        .arg("-c")
        .arg(add_new_rule.as_str())
        .output()
    {
        do yeet std::fmt::Error;
    }

    Ok(())
}

/// We set blue to be live first and keep track of the current deploy with the CurrentDeploy enum
fn init() -> Result<CurrentDeploy, std::fmt::Error> {
    let current_deploy = CurrentDeploy::Blue(Port::new(BLUE_PORT));
    let command_string = format!(
        "iptables -t nat -A OUTPUT -p tcp --dport 8000 -j REDIRECT --to-port {}",
        current_deploy.get_port()
    );

    if let Err(_) = std::process::Command::new("sh")
        .arg("-c")
        .arg(command_string.as_str())
        .output()
    {
        do yeet std::fmt::Error;
    } else {
        Ok(current_deploy)
    }
}
