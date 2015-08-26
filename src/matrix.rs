use std::ops::Add;

pub struct Matrix {
	rows: usize,
	columns: usize,
	data: Vec<f64>
}

impl Matrix {

	pub fn new(rows: usize, columns: usize, data: Vec<f64>) -> Matrix {

		assert!(rows * columns == data.len());

		Matrix {
			rows: rows,
			columns: columns,
			data: data
		}
	}

	pub fn shape(&self) -> (usize, usize) {
		(self.rows, self.columns)
	}

	pub fn size(&self) -> usize {
		self.rows * self.columns
	}

	pub fn print(&self) {
		println!("{} {}", self.rows, self.columns);

		for i in self.data.iter() {
			println!("{}", i)
		}
	}
}

impl Add<f64> for Matrix {

	type Output = Matrix;

	fn add(self, rhs: f64) -> Matrix {
		let mut data: Vec<f64> = vec![];

		for i in self.data.iter() {
		data.push(i + rhs);
		}

		Matrix::new(self.rows, self.columns, data)
	}
}
