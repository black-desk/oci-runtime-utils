pub fn default() -> serde_json::Value {
    let mut value: serde_json::Value = serde_json::from_str(include_str!("../misc/config.json"))
        .expect("builtin config is not a valid json file");

    value["linux"]["uidMappings"][0]["hostID"] = nix::unistd::getuid().as_raw().into();
    value["linux"]["gidMappings"][0]["hostID"] = nix::unistd::getuid().as_raw().into();
    value
}
