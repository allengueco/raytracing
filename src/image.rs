use crate::Num;

#[derive(Copy, Clone)]
pub struct Image {
    pub aspect_ratio: Num,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub(crate) fn from_width(aspect_ratio: Num, width: usize) -> Self {
        Self {
            aspect_ratio,
            width,
            height: (width as Num / aspect_ratio) as usize,
        }
    }
}
