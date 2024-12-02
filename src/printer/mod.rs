pub trait Printer:
    crate::cmds::inspect::Printer<crate::cmds::inspect::All>
    + crate::cmds::inspect::Printer<crate::cmds::inspect::MountInfo>
    + crate::cmds::inspect::Printer<crate::cmds::inspect::Capabilities>
    + crate::cmds::inspect::Printer<crate::cmds::inspect::Directory>
    + crate::cmds::inspect::Printer<crate::cmds::inspect::File>
{
}

mod json;
pub use json::Printer as Json;

mod yaml;
pub use yaml::Printer as YAML;

mod toml;
pub use toml::Printer as TOML;
