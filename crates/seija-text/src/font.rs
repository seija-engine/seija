use std::sync::Arc;

pub struct Font {
    swash: (u32, swash::CacheKey),
    data:Arc<dyn AsRef<[u8]> + Send + Sync>,
    id: fontdb::ID,
}

impl Font {
    pub fn new(info:&fontdb::FaceInfo) -> Option<Font> {
        let data = match &info.source {
            fontdb::Source::Binary(bin) => { Arc::clone(bin) },
            fontdb::Source::SharedFile(_path, data) => {  Arc::clone(data) },
            _ => panic!("Unsupported")
        };
        let swash = swash::FontRef::from_index((*data).as_ref(), info.index as usize)?;
        let swash_key = (swash.offset, swash.key);
        Some(Font {
            swash:swash_key,
            data,
            id:info.id
        })
    }
}