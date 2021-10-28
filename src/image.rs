use crate::Num;

#[derive(Copy, Clone)]
pub struct Image {
    pub aspect_ratio: AspectRatio,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub(crate) fn from_width(aspect_ratio: AspectRatio, width: usize) -> Self {
        Self {
            aspect_ratio,
            width,
            height: (width as Num / aspect_ratio.0) as usize,
        }
    }
}

#[derive(Copy, Clone)]
pub struct AspectRatio(pub Num);
