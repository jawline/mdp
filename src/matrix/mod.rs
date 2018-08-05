pub mod dense;
pub mod map;

use std::fmt::Display;

pub trait Matrix<T: Copy> {
	fn get(&self, x: usize, y: usize) -> Option<T>;
	fn set(&mut self, x: usize, y: usize, value: T) -> Option<()>;
	fn width(&self) -> usize;
	fn height(&self) -> usize;
}

pub fn print_matrix<T: Copy + Display>(mat: &Matrix<T>) {
	for y in 0..mat.height() {
		for x in 0..mat.width() {
			print!("{}{}", if x != 0 { "," } else { "" }, mat.get(x, y).unwrap());
		}
		print!("\n");
	}
}

pub use self::map::MapMatrix;
pub use self::dense::DenseMatrix;