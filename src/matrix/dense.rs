use matrix::Matrix;

pub struct DenseMatrix<T: Copy> {
	data: Vec<T>,
	width: usize,
	height: usize
}

impl<T: Copy> DenseMatrix<T> {

	pub fn new(default: T, width: usize, height: usize) -> DenseMatrix<T> {
		DenseMatrix {
			data: (0..width * height).map(|_| default).collect(),
			width: width,
			height: height
		}
	}

}

impl<T: Copy> Matrix<T> for DenseMatrix<T> {

	fn get(&self, x: usize, y: usize) -> Option<T> {
		if x < self.width && y < self.height {
			Some(self.data[(y * self.width) + x])
		} else {
			None
		}
	}

	fn set(&mut self, x: usize, y: usize, value: T) -> Option<()> {
		if x < self.width && y < self.height {
			self.data[(y * self.width) + x] = value;
			Some(())
		} else {
			None
		}
	}

	fn width(&self) -> usize { self.width }
	fn height(&self) -> usize { self.height } 

}