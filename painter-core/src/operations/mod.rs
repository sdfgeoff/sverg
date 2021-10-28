mod image_blit;
mod simple_transform;
mod bucket_fill;
mod stroke;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct OperationId(u128);


#[derive(PartialEq, Clone, Debug)]
pub enum Operation {
    // Stroke(StrokeData),
    NullOp,
    Fill(bucket_fill::FillData),
    ImageBlit(image_blit::ImageBlitData),
}
