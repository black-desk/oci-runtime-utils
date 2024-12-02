pub fn execute(output: &std::path::PathBuf) -> std::io::Result<()> {
    std::fs::write(output, crate::config::default().to_string())
}
