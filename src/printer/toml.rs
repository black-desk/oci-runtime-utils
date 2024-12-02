use std::io::Write;

pub struct Printer {}

impl Printer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> crate::cmds::inspect::Printer<T> for Printer
where
    T: serde::Serialize,
{
    fn print(&self, a: &T) -> std::io::Result<()> {
        std::io::stdout().write_all(toml::to_string_pretty(a).unwrap().as_bytes())?;
        Ok(())
    }
}

impl super::Printer for Printer {}
