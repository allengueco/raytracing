use crate::ray::Ray;
use crate::vec3::{Point3, Vector3};
use crate::Num;
use rand::{random, thread_rng, Rng};
use std::ops::Range;
use std::time::Duration;

pub struct Camera {
    pub origin: Point3,
    pub viewport: Viewport,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub u: Vector3,
    pub v: Vector3,
    lower_left_corner: Vector3,
    lens_radius: Num,
    focus_distance: Num,
    exposure: Range<Num>,
}

impl Camera {
    pub const ASPECT_RATIO: Num = 3. / 2.;
    pub const APERTURE: Num = 0.1;
    pub(crate) fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vector3,
        vfov: Num,
        exposure: Range<Num>,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport = {
            let height = 2. * h;
            let width = Camera::ASPECT_RATIO * height;
            Viewport { height, width }
        };

        let focus_distance = 10.0;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_distance * viewport.width * u;
        let vertical = focus_distance * viewport.height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focus_distance * w;

        Self {
            origin,
            viewport,
            horizontal,
            vertical,
            u,
            v,
            lower_left_corner,
            lens_radius: Self::APERTURE / 2.,
            focus_distance,
            exposure,
        }
    }

    pub(crate) fn llc(&self) -> Vector3 {
        self.lower_left_corner
    }

    pub(crate) fn cast_ray(&self, u: Num, v: Num) -> Ray {
        let rd = self.lens_radius * Vector3::random_in_unit_sphere(&mut rand::thread_rng());
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::from(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            rand::thread_rng().gen_range(self.exposure.clone()),
        )
    }
}

pub struct Viewport {
    pub width: Num,
    pub height: Num,
}
