//! Projection matrices that are intended to be used when the base coordinate system (i.e. the one used by the application code)
//! is right-handed with the the x-axis pointing right, y-axis pointing *up*, and z-axis pointing *out of the screen*.

/// Orthographic projection matrix.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space)and the destination space is left-handed
/// and y-up, with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn orthographic(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> [[f32; 4]; 4] {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let fmn = far - near;
    [
        [2.0 / rml, 0.0, 0.0, 0.0],
        [0.0, 2.0 / tmb, 0.0, 0.0],
        [0.0, 0.0, -1.0 / fmn, 0.0],
        [-(rpl / rml), -(tpb / tmb), -(near / fmn), 1.0],
    ]
}

/// Perspective projection matrix.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// left-handed and y-up with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> [[f32; 4]; 4] {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let nmf = z_near - z_far;

    [
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, z_far / nmf, -1.0],
        [0.0, 0.0, z_near * z_far / nmf, 0.0],
    ]
}

/// Perspective projection matrix with infinite z-far plane.
///
/// This is useful for extremely large scenes where having a far clip plane is extraneous anyway,
/// as allowing it to approach infinity it eliminates several approximate numerical computations
/// and so can improve z-fighting behavior.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// left-handed and y-up with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_infinite_z(vertical_fov: f32, aspect_ratio: f32, z_near: f32) -> [[f32; 4]; 4] {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;

    [
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, -1.0, -1.0],
        [0.0, 0.0, -z_near, 0.0],
    ]
}

/// Perspective projection matrix with reversed z-axis.
///
/// Reversed-Z provides significantly better precision and therefore reduced z-fighting
/// for most depth situations, especially when a floating-point depth buffer is used. You'll want to use
/// a reversed depth comparison function and depth clear value when using this projection.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// left-handed and y-up with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_reversed_z(
    vertical_fov: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,
) -> [[f32; 4]; 4] {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let nmf = z_near - z_far;

    [
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, -z_far / nmf - 1.0, -1.0],
        [0.0, 0.0, -z_near * z_far / nmf, 0.0],
    ]
}

/// Perspective projection matrix with reversed and infinite z-axis.
///
/// Reversed-Z provides significantly better precision and therefore reduced z-fighting
/// for most depth situations, especially when a floating-point depth buffer is used. You'll want to use
/// a reversed depth comparison function and depth clear value when using this projection.
///
/// Infinte-Z is useful for extremely large scenes where having a far clip plane is extraneous anyway,
/// as allowing it to approach infinity it eliminates several approximate numerical computations
/// and so can improve z-fighting behavior.
///
/// Combining them gives the best of both worlds for large scenes.
///
/// * `vertical_fov` should be provided in radians.
/// * `aspect_ratio` should be the quotient `width / height`.
///
/// This matrix is meant to be used when the source coordinate space is right-handed and y-up
/// (the standard computer graphics coordinate space) and the destination coordinate space is
/// left-handed and y-up with Z (depth) clip extending from 0.0 (close) to 1.0 (far).
#[inline]
pub fn perspective_reversed_infinite_z(
    vertical_fov: f32,
    aspect_ratio: f32,
    z_near: f32,
) -> [[f32; 4]; 4] {
    let t = (vertical_fov / 2.0).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;

    [
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, 0.0, -1.0],
        [0.0, 0.0, z_near, 0.0],
    ]
}
