mod bounding_box;
mod macros;
mod newton_solver;
mod quadratic_solver;
mod shape_projection;
mod vector_conversion;
mod wrapping_windows;

pub use bounding_box::BoundingBox;
pub use newton_solver::global_newton_solver;
pub use quadratic_solver::solve_quadratic;
pub use shape_projection::{DEdge, Edge, ShapeProjection};
pub use vector_conversion::{ToVec, ToVector};
pub use wrapping_windows::WrappingWindows;
