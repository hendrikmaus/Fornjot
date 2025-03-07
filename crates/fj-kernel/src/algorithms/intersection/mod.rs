//! Intersection algorithms

mod curve_face;
mod line_segment;
mod surface_surface;

pub use self::{
    curve_face::{CurveFaceIntersection, CurveFaceIntersectionList},
    line_segment::{line_segment, LineSegmentIntersection},
    surface_surface::surface_surface,
};
