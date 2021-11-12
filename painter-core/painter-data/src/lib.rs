mod v1;

pub use v1::*;
const CURRENT_FORMAT_VERSION: u32 = 1;



const HEAD_MAGIC_STR: &[u8] = b"PAINTER_SVEG";


#[derive(Debug)]
pub enum PainterDataError {
    WriteError(std::io::Error),
    SerializeError(std::boxed::Box<bincode::ErrorKind>),
    DeserializeError(std::boxed::Box<bincode::ErrorKind>),
    InvalidMagicString,
    UnknownVersion(u32),
    ReadError(std::io::Error),
}

pub fn write_into<W: std::io::Write>(image_to_write: &image::Image, mut writer: W) -> Result<(), PainterDataError> {
    writer.write_all(HEAD_MAGIC_STR).map_err(PainterDataError::WriteError)?;
    writer.write_all(&CURRENT_FORMAT_VERSION.to_le_bytes()).map_err(PainterDataError::WriteError)?;

    bincode::serialize_into(writer, image_to_write).map_err(PainterDataError::SerializeError)?;
    Ok(())
}

pub fn load_from_reader<R: std::io::Read>(mut reader: R) -> Result<image::Image, PainterDataError> {

    let mut magic_str = vec![0u8; HEAD_MAGIC_STR.len()];
    reader.read_exact(&mut magic_str).map_err(PainterDataError::ReadError)?;
    
    if magic_str != HEAD_MAGIC_STR {
        return Err(PainterDataError::InvalidMagicString);
    }
    let mut version_data = [0u8; 4];
    reader.read_exact(&mut version_data).map_err(PainterDataError::ReadError)?;
    let version_number = u32::from_le_bytes(version_data);

    match version_number {
        1 => {
            let img = bincode::deserialize_from(reader).map_err(PainterDataError::DeserializeError)?;
            Ok(img)
        }
        number => {
            Err(PainterDataError::UnknownVersion(number))
        }
    }

}