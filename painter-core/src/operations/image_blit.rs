/// An ImageBlit operation dumps an image directly onto the canvas.
/// The image is scaled such that it spans the entire canvas (from 0.0 to 1.0)
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ImageBlitData {
    /// The raw image data
    image_data: Vec<u8>
}