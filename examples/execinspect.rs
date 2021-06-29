use docker_api::{Docker, Exec, ExecContainerOptions};
use futures::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let mut args = env::args().skip(1);

    // First argument is container id
    let id = args.next().expect("You need to specify a container id");
    // Rest is command to run in the container
    let cmd = args.collect::<Vec<String>>();
    println!("{} {:?}", id, cmd);

    // Create options with specified command
    let opts = ExecContainerOptions::builder()
        .cmd(cmd)
        .attach_stdout(true)
        .attach_stderr(true)
        .build();

    let exec = Exec::create(&docker, &id, &opts).await?;

    println!("{:#?}", exec.inspect().await?);

    let mut stream = exec.start();

    stream.next().await;

    println!("{:#?}", exec.inspect().await?);

    Ok(())
}
