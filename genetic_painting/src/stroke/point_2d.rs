#[derive(Clone, Default)]
pub struct Point2D {
    pub x: u32,
    pub y: u32,
}

impl Point2D {
	pub fn as_tuple(&self) -> (f32, f32) {
		return ( self.x as f32, self.y as f32);
	}
}
