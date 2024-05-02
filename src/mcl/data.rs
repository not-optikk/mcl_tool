use binrw::binrw;
use binrw::BinReaderExt;
use binrw::NullString;
use binrw::io::Cursor;
use crate::util::fixed::FixedPoint;

#[binrw]
pub struct Vec4 {
	#[br(map = |x: u32| FixedPoint::new(x).to_f64(1, 19, 12))]
	pub x: f64,
	#[br(map = |x: u32| FixedPoint::new(x).to_f64(1, 19, 12))]
	pub y: f64,
	#[br(map = |x: u32| FixedPoint::new(x).to_f64(1, 19, 12))]
	pub z: f64,
	#[br(map = |x: u32| FixedPoint::new(x).to_f64(1, 19, 12))]
	pub w: f64
}

#[binrw]
pub struct Face {
	// Index of the first vertex
	pub vertex0: u16,
	// Index of the second vertex
	pub vertex1: u16,
	// Index of the third vertex
	pub vertex2: u16,
	// Object index that this face belongs to
	pub object_id: u16,
	// Normal vector for this face
	pub normal: Vec4
}

#[binrw]
#[brw(magic = b"MCL ")]
pub struct Mcl {
	pub file_version: u16,

	#[br(align_before = 0x30)]
	pub name: NullString,

	#[br(align_before = 0x48)]
	pub faces_offset: u32,
	pub face_count: u32,

	#[br(align_before = 0x68)]
	pub vertices_offset: u32,
	pub vertex_count: u32,

	#[br(align_before = vertices_offset)]
	#[br(count = vertex_count)]
	pub vertices: Vec<Vec4>,

	#[br(align_before = faces_offset)]
	#[br(count = face_count)]
	pub faces: Vec<Face>,

	#[br(align_before = 0x70)]
	pub unknown_offset0: u32,
	pub unknown_count0: u32,

	#[br(align_before = 0x50)]
	pub unknown_offset1: u32
}

pub fn read_bin(data: Vec<u8>) -> Mcl {
	let mut reader = Cursor::new(data);
	let mcl: Mcl = reader.read_le().unwrap();

	mcl
}