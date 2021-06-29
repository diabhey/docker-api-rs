use docker_api::Docker;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let id = env::args()
        .nth(1)
        .expect("You need to specify a network id");

    match docker.networks().get(&id).inspect().await {
        Ok(network_info) => println!("{:#?}", network_info),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
