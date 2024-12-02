pub mod filesystem;

#[derive(serde::Serialize)]
pub struct Capabilities {
    ambient: caps::CapsHashSet,
    bounding: caps::CapsHashSet,
    effective: caps::CapsHashSet,
    inheritable: caps::CapsHashSet,
    permitted: caps::CapsHashSet,
}

impl Capabilities {
    pub fn new() -> std::io::Result<Self> {
        Ok(Capabilities {
            ambient: caps::read(None, caps::CapSet::Ambient).unwrap(),
            bounding: caps::read(None, caps::CapSet::Bounding).unwrap(),
            effective: caps::read(None, caps::CapSet::Effective).unwrap(),
            inheritable: caps::read(None, caps::CapSet::Inheritable).unwrap(),
            permitted: caps::read(None, caps::CapSet::Permitted).unwrap(),
        })
    }
}
