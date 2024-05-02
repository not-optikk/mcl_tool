use crate::mcl::data::read_bin;
use crate::obj::writer::WavefrontWriter;
use crate::obj::data::{Face, Group, Vertex};
use core::panic;
use std::collections::HashSet;
use std::path::PathBuf;
use std::{fs, io::Error};

pub fn main(input_file: &PathBuf, output_directory: &PathBuf) -> Result<(), Error> {
	let output_file_name = match input_file.file_name() {
		Some(file_name) => file_name,
		None => panic!("Expected file as input argument")
	};

	match output_directory.extension() {
		Some(ext) => panic!("Expected an output directory argument, but path has a file extension: {:?}", ext),
		None => ()
	};

	if output_directory.is_dir() {
		fs::create_dir_all(output_directory).expect(&format!("Could not create non-existent directory: {}", output_directory.display()));
	};

	let mcl_file = std::fs::read(&input_file).expect("Could not read input file.");
	let mcl = read_bin(mcl_file);

	let mut found_group_set: HashSet<u16> = HashSet::new();
	let mut obj_writer = WavefrontWriter::new();	
	
	for vert in mcl.vertices {
		obj_writer.add_vertex(Vertex {
			x: vert.x as f32,
			y: vert.y as f32,
			z: vert.z as f32,
			w: vert.w as f32
		});
	}

	for face in &mcl.faces {
		// The mcl file stores "groups" per-face, and the best way I could think to translate this is by adding
		// separate objects in the obj file.
		if !found_group_set.contains(&face.object_id) {
			let group = Group {
				faces: Vec::new()
			};
			obj_writer.add_group(group);
			found_group_set.insert(face.object_id);
		}
	}

	for face in &mcl.faces {
		// The mcl file stores normals for the entire face in one vector. I decided to add that to the obj file's
		// vertex normals for the face definition to index for each of its own vertices. 
		obj_writer.add_normal(Vertex {
			x: face.normal.x as f32,
			y: face.normal.y as f32,
			z: face.normal.z as f32,
			w: face.normal.w as f32
		});
		
		let vertex_normal_index = obj_writer.normals.len();
		let group = obj_writer.groups.get_mut(face.object_id as usize).unwrap();

		group.faces.push(Face {
			vertices: vec![ face.vertex0 as usize, face.vertex1 as usize, face.vertex2 as usize ],
			normals: vec![ vertex_normal_index, vertex_normal_index, vertex_normal_index ]
		})
	}

	
	let output_file_path = output_directory.join(&format!("{}.obj", output_file_name.to_str().unwrap_or("output")));
	match obj_writer.write(&output_file_path) {
		Ok(()) => {
			println!("Wrote to: {}", output_file_path.canonicalize().unwrap().display());
			println!("Faces: {}, Vertices: {}", mcl.face_count, mcl.vertex_count);
		},
		Err(err) => {
			println!("Error: {}", err);
			panic!("Could not write to {}", output_file_path.canonicalize().unwrap().display());
		}
	};

	Ok(())
}