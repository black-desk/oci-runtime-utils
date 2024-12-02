pub fn enable_utils() -> json_patch::Patch {
    serde_json::from_str(
        format!(
            r#"[ {{
                "op": "add",
                "path": "/mounts/-",
                "value": {{
                    "source": "{}",
                    "destination": "/usr/local/bin/oci-runtime-utils",
                    "options": [
                        "bind",
                        "ro"
                    ]
                }}
            }} ]"#,
            std::env::current_exe().unwrap().to_str().unwrap()
        )
        .as_str(),
    )
    .unwrap()
}

pub fn with_command(command: &Vec<String>) -> json_patch::Patch {
    serde_json::from_value(serde_json::json!([
      { "op": "replace", "path": "/process/args", "value": &command },
    ]))
    .unwrap()
}
