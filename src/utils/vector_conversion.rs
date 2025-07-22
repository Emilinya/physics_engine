use bevy::math::{DVec2, Vec2};
use nalgebra::Vector2;

pub trait ToVec {
    type Vec;

    fn to_vec(self) -> Self::Vec;
}

pub trait ToVector {
    type Vector;

    fn to_vector(self) -> Self::Vector;
}

impl ToVec for Vector2<f32> {
    type Vec = Vec2;

    fn to_vec(self) -> Self::Vec {
        Vec2::new(self.x, self.y)
    }
}

impl ToVec for Vector2<f64> {
    type Vec = DVec2;

    fn to_vec(self) -> Self::Vec {
        DVec2::new(self.x, self.y)
    }
}

impl ToVector for Vec2 {
    type Vector = Vector2<f32>;

    fn to_vector(self) -> Self::Vector {
        Vector2::new(self.x, self.y)
    }
}

impl ToVector for DVec2 {
    type Vector = Vector2<f64>;

    fn to_vector(self) -> Self::Vector {
        Vector2::new(self.x, self.y)
    }
}
