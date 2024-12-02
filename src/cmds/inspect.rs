fn get_mountinfo() -> std::io::Result<crate::inspect::filesystem::File> {
    let path: std::path::PathBuf = ["proc", "self", "mountinfo"].iter().collect();
    crate::inspect::filesystem::File::new(path)
}

fn get_procfs() -> std::io::Result<crate::inspect::filesystem::Directory> {
    let path: std::path::PathBuf = ["proc"].iter().collect();
    crate::inspect::filesystem::Directory::new(path)
}

fn get_sysfs() -> std::io::Result<crate::inspect::filesystem::Directory> {
    let path: std::path::PathBuf = ["sys"].iter().collect();
    crate::inspect::filesystem::Directory::new(path)
}

fn get_dev() -> std::io::Result<crate::inspect::filesystem::Directory> {
    let path: std::path::PathBuf = ["dev"].iter().collect();
    crate::inspect::filesystem::Directory::new(path)
}

fn get_capabilities() -> std::io::Result<crate::inspect::Capabilities> {
    crate::inspect::Capabilities::new()
}

pub trait Printer<T> {
    fn print(&self, a: &T) -> std::io::Result<()>;
}

#[derive(serde::Serialize)]
pub struct All {
    pub mountinfo: crate::inspect::filesystem::File,
    pub procfs: crate::inspect::filesystem::Directory,
    pub sysfs: crate::inspect::filesystem::Directory,
    pub dev: crate::inspect::filesystem::Directory,
    pub capabilities: crate::inspect::Capabilities,
}

pub fn all(printer: &dyn Printer<All>) -> std::io::Result<()> {
    printer.print(&All {
        mountinfo: get_mountinfo().unwrap(),
        procfs: get_procfs().unwrap(),
        sysfs: get_sysfs().unwrap(),
        dev: get_dev().unwrap(),
        capabilities: get_capabilities().unwrap(),
    })
}

#[derive(serde::Serialize)]
pub struct MountInfo {
    pub mountinfo: crate::inspect::filesystem::File,
}

pub fn mountinfo(printer: &dyn Printer<MountInfo>) -> std::io::Result<()> {
    printer.print(&MountInfo {
        mountinfo: get_mountinfo().unwrap(),
    })
}

#[derive(serde::Serialize)]
pub struct Capabilities {
    pub capabilities: crate::inspect::Capabilities,
}

pub fn capabilities(printer: &dyn Printer<Capabilities>) -> std::io::Result<()> {
    let caps = crate::inspect::Capabilities::new()?;
    printer.print(&Capabilities { capabilities: caps })
}

#[derive(serde::Serialize)]
pub struct Directory {
    pub directory: crate::inspect::filesystem::Directory,
}

pub fn directory(printer: &dyn Printer<Directory>, path: &str) -> std::io::Result<()> {
    let path = std::path::PathBuf::from(path);
    let directory = crate::inspect::filesystem::Directory::new(path)?;
    printer.print(&Directory { directory })
}

#[derive(serde::Serialize)]
pub struct File {
    pub file: crate::inspect::filesystem::File,
}
pub fn file(printer: &dyn Printer<File>, path: &str) -> std::io::Result<()> {
    printer.print(&File {
        file: crate::inspect::filesystem::File::new(std::path::PathBuf::from(path))?,
    })
}
