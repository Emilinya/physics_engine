#![allow(dead_code)]

use bevy::math::DVec2;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: DVec2,
    pub max: DVec2,
}

impl BoundingBox {
    #[inline]
    pub fn from_corners(p0: DVec2, p1: DVec2) -> Self {
        Self {
            min: p0.min(p1),
            max: p0.max(p1),
        }
    }

    #[inline]
    pub fn from_center_size(origin: DVec2, size: DVec2) -> Self {
        assert!(size.cmpge(DVec2::ZERO).all(), "Rect size must be positive");
        let half_size = size / 2.;
        Self {
            min: origin - half_size,
            max: origin + half_size,
        }
    }

    #[inline]
    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn size(&self) -> DVec2 {
        self.max - self.min
    }

    #[inline]
    pub fn center(&self) -> DVec2 {
        (self.min + self.max) * 0.5
    }

    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        let x_intersects = self.max.x > other.min.x && other.max.x > self.min.x;
        let y_intersects = self.max.y > other.min.y && other.max.y > self.min.y;

        x_intersects && y_intersects
    }
}
