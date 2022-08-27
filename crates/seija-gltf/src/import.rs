use std::path::Path;
use seija_core::{anyhow::{Result,anyhow}};
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Scheme<'a> {
    /// `data:[<media type>];base64,<data>`.
    Data(Option<&'a str>, &'a str),

    /// `file:[//]<absolute file path>`.
    File(&'a str),

    /// `../foo`, etc.
    Relative,

    /// Placeholder for an unsupported URI scheme identifier.
    Unsupported,
}

impl<'a> Scheme<'a> {
    pub(crate) fn parse(uri: &str) -> Scheme<'_> {
        if uri.contains(':') {
            if let Some(rest) = uri.strip_prefix("data:") {
                let mut it = rest.split(";base64,");

                match (it.next(), it.next()) {
                    (match0_opt, Some(match1)) => Scheme::Data(match0_opt, match1),
                    (Some(match0), _) => Scheme::Data(None, match0),
                    _ => Scheme::Unsupported,
                }
            } else if let Some(rest) = uri.strip_prefix("file://") {
                Scheme::File(rest)
            } else if let Some(rest) = uri.strip_prefix("file:") {
                Scheme::File(rest)
            } else {
                Scheme::Unsupported
            }
        } else {
            Scheme::Relative
        }
    }

    pub(crate) fn read(uri: &str,base_path:&Path) -> Result<Vec<u8>> {
        match Scheme::parse(uri) {
            Scheme::Data(_, base64) => base64::decode(&base64).map_err(|e|anyhow!(e)),
            Scheme::File(path) => read_to_end(path),
            Scheme::Relative  => read_to_end(base_path.join(uri)),
            Scheme::Unsupported => Err(anyhow!(gltf::Error::UnsupportedScheme)),
        }
    }
}

fn read_to_end<P>(path: P) -> Result<Vec<u8>> where P: AsRef<Path> {
    use std::io::Read;
    let file = std::fs::File::open(path.as_ref()).map_err(|e| anyhow!(e))?;
   
    let length = file.metadata().map(|x| x.len() + 1).unwrap_or(0);
    let mut reader = std::io::BufReader::new(file);
    let mut data = Vec::with_capacity(length as usize);
    reader.read_to_end(&mut data).map_err(|e| anyhow!(e))?;
    Ok(data)
}