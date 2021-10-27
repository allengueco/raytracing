use crate::Num;
use crate::vec3::{Point3, Vector3};
use crate::image::AspectRatio;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Point3,
    pub viewport: Viewport,
    focal_length: Num,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    lower_left_corner: Vector3,
}

impl Camera {
    pub(crate) fn new(viewport: Viewport, origin: Point3) -> Self {
        let focal_length = 1.0;
        let horizontal = Vector3::new(viewport.width as Num, 0., 0.);
        let vertical = Vector3::new(0., viewport.height as Num, 0.);
        let lower_left_corner = origin - horizontal/2. - vertical/2. - Vector3::new(0., 0., focal_length);
        Self {
            origin,
            viewport,
            focal_length,
            horizontal,
            vertical,
            lower_left_corner
        }
    }

    pub(crate) fn llc(&self) -> Vector3 {
        self.lower_left_corner
    }

    pub(crate) fn ray(&self, u: Num, v: Num) -> Ray {
        Ray::from(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}

#[derive(Copy, Clone)]
pub struct Viewport {
    pub width: usize,
    pub height: usize,
}

impl Viewport {
    pub(crate) fn from_width(aspect_ratio: AspectRatio, width: usize) -> Self {
        Self {
            width,
            height: (width as Num / aspect_ratio.0) as usize
        }
    }

    pub(crate) fn from_height(aspect_ratio: AspectRatio, height: usize) -> Self {
        Self {
            width: (aspect_ratio.0 * height as Num) as usize,
            height
        }
    }
}
