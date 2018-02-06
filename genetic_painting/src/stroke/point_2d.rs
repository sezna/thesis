#[derive(Clone, Default)]


/// Struct to represent a 2d point.
pub struct Point2D {
    pub x: u32,
    pub y: u32,
}

impl Point2D {
	
	/// Return the 2d point as a tuple of floats.
	pub fn as_tuple(&self) -> (f32, f32) {
		return ( self.x as f32, self.y as f32);
	}
}
