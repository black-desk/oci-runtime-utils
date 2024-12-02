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
        serde_yaml::to_writer(std::io::stdout(), a).unwrap();
        Ok(())
    }
}

impl super::Printer for Printer {}
