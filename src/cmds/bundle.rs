pub fn execute(bundle: &std::path::PathBuf) -> std::io::Result<()> {
    if !bundle.exists() {
        std::fs::create_dir_all(bundle)?;
    }
    let mut config = crate::config::default();
    let enable_utils = crate::patch::enable_utils();
    json_patch::patch(&mut config, &enable_utils).unwrap();
    let with_commands = crate::patch::with_command(&vec![
        "/usr/local/bin/oci-runtime-utils".to_string(),
        "true".to_string(),
    ]);
    json_patch::patch(&mut config, &with_commands).unwrap();
    std::fs::write("config.json", config.to_string())?;
    std::fs::create_dir(bundle.join("rootfs"))?;
    Ok(())
}
