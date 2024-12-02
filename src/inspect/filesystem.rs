use std::os::{linux::fs::MetadataExt, unix::fs::PermissionsExt};

#[derive(serde::Serialize, Debug)]
pub struct Metadata {
    pub permissions: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub accessed: u64,
    pub modified: u64,
}

impl Metadata {
    pub fn new(metadata: std::fs::Metadata) -> std::io::Result<Self> {
        Ok(Self {
            permissions: metadata.permissions().mode(),
            uid: metadata.st_uid(),
            gid: metadata.st_gid(),
            size: metadata.len(),
            accessed: metadata
                .accessed()?
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified: metadata
                .modified()?
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
}

#[derive(serde::Serialize, Debug)]
pub struct File {
    pub path: std::path::PathBuf,
    pub metadata: Metadata,
    pub content: String,
}

impl File {
    pub fn new(path: std::path::PathBuf) -> std::io::Result<Self> {
        Ok(Self {
            content: std::fs::read_to_string(&path)?,
            metadata: Metadata::new(path.metadata()?)?,
            path,
        })
    }
}

#[derive(serde::Serialize, Debug)]
pub struct DirectoryEntry {
    pub path: std::path::PathBuf,
    pub metadata: Metadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<std::path::PathBuf>,
}

#[derive(serde::Serialize, Debug)]
pub struct Directory {
    pub path: std::path::PathBuf,
    pub entries: Vec<DirectoryEntry>,
}

impl Directory {
    pub fn new(path: std::path::PathBuf) -> std::io::Result<Self> {
        let mut entries = Vec::<DirectoryEntry>::new();
        let walker = walkdir::WalkDir::new(&path);
        for entry in walker {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    log::error!("Error: {}", e);
                    continue;
                }
            };

            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(e) => {
                    log::error!("Error: {}", e);
                    continue;
                }
            };

            let destination = metadata.file_type().is_symlink().then(|| {
                std::fs::read_link(entry.path()).unwrap_or_else(|e| {
                    log::error!("Error: {}", e);
                    std::path::PathBuf::new()
                })
            });

            let metadata = Metadata::new(metadata)?;

            let entry = DirectoryEntry {
                path: entry.path().to_path_buf(),
                destination,
                metadata,
            };

            entries.push(entry);
        }
        Ok(Self { path, entries })
    }
}
