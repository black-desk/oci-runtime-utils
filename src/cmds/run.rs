pub fn execute(runtime: &std::path::PathBuf, container_id: &str) -> std::io::Result<()> {
    if !runtime.exists() {
        log::error!("OCI runtime not found: {:?}", runtime);
        std::process::exit(1);
    }

    log::info!("Run container with runtime: {:?}", runtime);

    let mut command = std::process::Command::new(runtime);
    command
        .arg("run")
        .arg("--bundle")
        .arg(".")
        .arg(container_id);

    match command
        .status()
        .expect("failed to execute process")
        .success()
    {
        true => Ok(()),
        false => Err(std::io::Error::other("OCI runtime exited with error")),
    }
}
