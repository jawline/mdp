use matrix::Matrix;
use std::collections::BTreeMap;

pub struct MapMatrix<T: Copy> {
	data: BTreeMap<usize, BTreeMap<usize, T>>,
	default: T,
	width: usize,
	height: usize
}

impl<T: Copy> MapMatrix<T> {

	pub fn new(default: T, width: usize, height: usize) -> MapMatrix<T> {
		MapMatrix {
			data: BTreeMap::new(),
			default: default,
			width: width,
			height: height
		}
	}

}

impl<T: Copy> Matrix<T> for MapMatrix<T> {

	fn get(&self, x: usize, y: usize) -> Option<T> {
		if x > self.width || y > self.height {
			None
		} else {
			match self.data.get(&x) {
				Some(row) => {
					match row.get(&y) {
						Some(&item) => Some(item),
						None => Some(self.default)
					}
				},
				None => Some(self.default)
			}
		}
	}

	fn set(&mut self, x: usize, y: usize, value: T) -> Option<()> {

		self.data
			.entry(x)
			.or_insert(BTreeMap::new())
			.insert(y, value);

		Some(())
	}

	fn width(&self) -> usize { self.width }
	fn height(&self) -> usize { self.height } 

}