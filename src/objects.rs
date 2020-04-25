use na::{Point3, Vector3};

use crate::ray::Ray;
use crate::lighting::Color;

pub trait Object {
    fn distance_estimate(&self, point: Point3<f32>) -> f32;
    // returns closest surface normal
    fn get_normal(&self, point: Point3<f32>) -> Vector3<f32>;
    fn get_color_ref(&self) -> &Color;
    fn get_type_name(&self) -> &'static str;
}

#[derive(Debug)]
pub struct Sphere {
    pub centre: Point3<f32>,
    pub radius: f32,
    pub color: Color,
}

impl Object for Sphere {
    fn distance_estimate(&self, point: Point3<f32>) -> f32 {
        // vector to centre of sphere
        let r_centre = self.centre - point;
        // distance is then magnitude of this vector, take away the radius of the sphere
        r_centre.norm() - self.radius
    }

    fn get_normal(&self, point: Point3<f32>) -> Vector3<f32> {
        // vector from sphere to point normalised = surface normal
        (point - self.centre).normalize()
    }

    fn get_color_ref(&self) -> &Color {
        &self.color
    }

    fn get_type_name(&self) -> &'static str {
        "Sphere"
    }
}

#[derive(Debug)]
pub struct HorizontalPlane {
    pub y: f32,
    pub color: Color,
}

impl Object for HorizontalPlane {
    fn distance_estimate(&self, point: Point3<f32>) -> f32 {
        // Get cosine squared of angle to plane via dot product: j * r/|r| = cos(a) = 1 * r.y/r
        let cos_ang_squared = (point.y).powi(2)/Vector3::new(point.x, point.y, point.z).norm_squared();
        // sin^2 + cos^2 = 1 -> sin = sqrt(1 - cos^2)
        // dy/sin(a) = distance
        (point.y - self.y).abs()/(1.0 - cos_ang_squared).sqrt()

        // (point.y - self.y).abs()
    }

    // Simple upwards vector
    fn get_normal(&self, point: Point3<f32>) -> Vector3<f32> {
        Vector3::new(0.0, 1.0, 0.0)
    }

    fn get_color_ref(&self) -> &Color {
        &self.color
    }

    fn get_type_name(&self) -> &'static str {
        "HorizontalPlane"
    }
}