pub fn execute(
    input: &std::path::PathBuf,
    output: &std::path::PathBuf,
    patch: &Option<String>,
    with_utils: bool,
    commands: &Vec<String>,
) -> std::io::Result<()> {
    let mut config: serde_json::Value =
        serde_json::from_str(std::fs::read_to_string(input).unwrap().as_str())?;

    if with_utils {
        let patch = crate::patch::enable_utils();
        json_patch::patch(&mut config, &patch).unwrap();
    }

    if !commands.is_empty() {
        let patch = crate::patch::with_command(commands);
        json_patch::patch(&mut config, &patch).unwrap();
    }

    if let Some(patch_file) = patch {
        let patch: json_patch::Patch =
            serde_json::from_str(std::fs::read_to_string(patch_file).unwrap().as_str())?;
        json_patch::patch(&mut config, &patch).unwrap();
    }

    std::fs::write(output, format!("{}", config))
}
