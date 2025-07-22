use bevy::math::{DVec2, Vec2};

macro_rules! define_edge {
    ($name:ident, $vec_type:ty) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name<'a> {
            v1: &'a $vec_type,
            v2: &'a $vec_type,
        }

        impl<'a> $name<'a> {
            #[inline]
            pub fn new(v1: &'a $vec_type, v2: &'a $vec_type) -> Self {
                Self { v1, v2 }
            }

            #[inline]
            pub fn tangent(&self) -> $vec_type {
                let between = self.v2 - self.v1;
                <$vec_type>::new(between.y, -between.x)
            }

            #[inline]
            #[allow(dead_code)]
            pub fn point_outside(&self, point: $vec_type) -> bool {
                self.tangent().dot(point) > 0.0
            }
        }
    };
}

define_edge! {Edge, Vec2}
define_edge! {DEdge, DVec2}

#[derive(Debug, Clone, Copy)]
pub struct ShapeProjection {
    min: f32,
    max: f32,
}

impl ShapeProjection {
    #[inline]
    pub fn from_min_max(min: f32, max: f32) -> Self {
        debug_assert!(min < max);
        Self { min, max }
    }

    pub fn project_vertices(vertices: &[Vec2], tangent: Vec2) -> Self {
        let mut min = f32::INFINITY;
        let mut max = -f32::INFINITY;

        for vertex in vertices {
            let projection = vertex.dot(tangent);
            min = f32::min(min, projection);
            max = f32::max(max, projection);
        }

        Self { min, max }
    }

    #[inline]
    pub fn overlap(&self, other: &Self) -> f32 {
        -f32::max(other.min - self.max, self.min - other.max)
    }
}
