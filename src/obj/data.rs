pub struct Vertex {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

pub struct Face {
	pub vertices: Vec<usize>,
	pub normals: Vec<usize>
}

pub struct Group {
	pub faces: Vec<Face>
}