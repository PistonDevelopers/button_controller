//! Various useful methods for math.

use std::time::Duration;

use vecmath;
use vecmath::mat2x3_inv as inv;
use vecmath::row_mat2x3_transform_pos2 as transform_pos;

/// The type used for scalars.
pub type Scalar = f64;

/// The type used for matrices.
pub type Matrix2d<T = Scalar> = vecmath::Matrix2x3<T>;

/// Rectangle dimensions: [x, y, w, h]
pub type Rectangle<T = Scalar> = [T; 4];

/// The type used for 2D vectors.
pub type Vec2d<T = Scalar> = vecmath::Vector2<T>;

/// Returns true if transformed point is inside rectangle.
pub fn is_inside(pos: Vec2d, transform: Matrix2d, rect: Rectangle) -> bool {
    let inv = inv(transform);
    let pos = transform_pos(inv, pos);
    pos[0] >= rect[0] && pos[1] >= rect[1] && pos[0] < rect[0] + rect[2] &&
    pos[1] < rect[1] + rect[3]
}

/// Returns the number of seconds of duration.
pub fn duration_to_secs(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1.0e9 + duration.subsec_nanos() as f64 / 1.0e9
}
