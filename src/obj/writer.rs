use std::fs::File;
use std::io::{Result, Write};
use std::path::PathBuf;
use super::data::{Face, Group, Vertex};

pub struct WavefrontWriter {
	pub vertices: Vec<Vertex>,
	pub normals: Vec<Vertex>,
	pub faces: Vec<Face>,
	pub groups: Vec<Group>
}

impl WavefrontWriter {
	pub fn new() -> Self {
		let writer = Self {
			vertices: Vec::new(),
			normals: Vec::new(),
			faces: Vec::new(),
			groups: Vec::new()
		};

		writer
	}

	pub fn add_vertex(&mut self, vertex: Vertex) {
		self.vertices.push(vertex);
	}

	pub fn add_face(&mut self, face: Face) {
		self.faces.push(face);
	}

	pub fn add_normal(&mut self, normal: Vertex) {
		self.normals.push(normal);
	}

	pub fn add_group(&mut self, group: Group) {
		self.groups.push(group);
	}

	pub fn write(&self, file_path: &PathBuf) -> Result<()> {
		let mut file = File::create(file_path)?;

		for vertex in &self.vertices {
			writeln!(file, "v {} {} {} {}", vertex.x, vertex.y, vertex.z, vertex.w)?;
		}

		for normal in &self.normals {
			writeln!(file, "vn {} {} {}", normal.x, normal.y, normal.z)?;
		}

		for face in &self.faces {
			write!(file, "f")?;
			for (vertex_index, normal_index) in face.vertices.iter().zip(&face.normals) {
				write!(file, " {}/{}", vertex_index + 1, normal_index + 1)?;
			}
			writeln!(file)?;
		}

		for (pos, group) in self.groups.iter().enumerate() {
			writeln!(file, "o {}", pos)?;
			for face in &group.faces {
				write!(file, "f")?;
				for (vertex_index, normal_index) in face.vertices.iter().zip(&face.normals) {
					write!(file, " {}/{}", vertex_index + 1, normal_index + 1)?;
				}
				writeln!(file)?;
			}
		}

		Ok(())
	}
}