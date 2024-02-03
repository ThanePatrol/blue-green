fn main() {
    //start docker containers
    std::process::Command::new("sh")
        .arg("-c")
        .arg("cd dummy-service && docker-compose up -d");

    //set blue to be live first

    let blue = "http://localhost:8080";
    println!("Hello, world!");
}
