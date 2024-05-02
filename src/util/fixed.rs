pub struct FixedPoint<T> {
	value: T
}

impl FixedPoint<u32> {
	pub fn new(value: u32) -> Self {
		FixedPoint { value }
	}

	pub fn to_f64(self, sign_bits: u32, int_bits: u32, frac_bits: u32) -> f64 {
		let x = self.value;
		assert!(sign_bits <= 1);
		assert!(int_bits + frac_bits > 0);
		assert!(sign_bits + int_bits + frac_bits <= 32);

		let y = if sign_bits == 0 {
			x as f64
		} else {
			let sign_mask = (1 << (int_bits + frac_bits)) as u32;
			if x & sign_mask != 0 {
				(x | !(sign_mask - 1)) as i32 as f64
			} else {
				x as f64
			}
		};
		y * 0.5f64.powi(frac_bits as i32)
	}
}