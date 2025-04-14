use bevy::math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Edge<'a> {
    v1: &'a Vec2,
    v2: &'a Vec2,
}

impl<'a> Edge<'a> {
    #[inline]
    pub fn new(v1: &'a Vec2, v2: &'a Vec2) -> Self {
        Self { v1, v2 }
    }

    #[inline]
    pub fn tangent(&self) -> Vec2 {
        let between = self.v2 - self.v1;
        Vec2::new(between.y, -between.x)
    }

    #[inline]
    pub fn point_outside(&self, point: Vec2) -> bool {
        self.tangent().dot(point) > 0.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeProjection {
    min: f32,
    max: f32,
}

impl ShapeProjection {
    #[expect(dead_code)]
    pub fn new(min: f32, max: f32) -> Self {
        debug_assert!(min < max);
        Self { min, max }
    }

    pub fn orthogonal_to_edge(edge: &Edge, vertices: &[Vec2]) -> Self {
        let tangent = edge.tangent().normalize();
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
    pub fn overlaps_with(&self, other: &Self) -> bool {
        self.max > other.min && other.max > self.min
    }
}
