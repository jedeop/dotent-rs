pub enum Asset {
    Image(AssetData),
    Sound(AssetData),
}

pub struct AssetData {
    pub(crate) data: Vec<u8>,
    pub(crate) name: String,
    pub(crate) ext: String,
}

impl AssetData {
    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn ext(&self) -> &str {
        self.ext.as_ref()
    }
}
