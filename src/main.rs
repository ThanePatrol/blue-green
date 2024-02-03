#![feature(yeet_expr)]

use std::{process::exit, time::Duration};

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

/// This program shows

//NB - this requires to be run as root for the iptables commands to succeed
fn main() {
    //start docker containers
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("cd dummy-service && docker-compose up -d")
        .output()
        .expect("error running docker - is it installed and docker daemon running?");

    let iptables_error_string = "Error running iptables, are you root?";

    if let Ok(mut current_deploy) = init() {
        let mut buffer = String::new();

        // we read from stdin and change listener rules as appropriate
        loop {
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("Error reading from stdin");

            match buffer.as_str() {
                "green" => {
                    switch_ip_tables(&mut current_deploy, "green").expect(iptables_error_string)
                }
                "blue" => {
                    switch_ip_tables(&mut current_deploy, "blue").expect(iptables_error_string)
                }
                "exit" => exit(1),
                _ => println!("Enter one of the following strings: 'blue' 'green' or 'exit'"),
            }
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
        current_deploy.get_port()
    );
    match current_deploy {
        CurrentDeploy::Blue(_) => *current_deploy = CurrentDeploy::Green(Port::new(8002)),
        CurrentDeploy::Green(_) => *current_deploy = CurrentDeploy::Blue(Port::new(8001)),
    }

    //TODO - does this need to be 1 second
    std::thread::sleep(Duration::from_secs(1));

    let delete_current_rule = format!(
        "iptables -t nat -A OUTPUT -p tcp --dport 8000 -j REDIRECT --to-port {}",
        current_deploy.get_port()
    );

    if let Err(_) = std::process::Command::new("sh")
        .arg("-c")
        .arg(delete_current_rule.as_str())
        .output()
    {
        do yeet std::fmt::Error;
    }

    if let Err(_) = std::process::Command::new("sh")
        .arg("-c")
        .arg(delete_current_rule.as_str())
        .output()
    {
        do yeet std::fmt::Error;
    }

    Ok(())
}

/// We set blue to be live first and keep track of the current deploy with the CurrentDeploy enum
fn init() -> Result<CurrentDeploy, std::fmt::Error> {
    let current_deploy = CurrentDeploy::Blue(Port::new(8001));
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
