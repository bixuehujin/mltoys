use std::ops::{Add, Sub};
use std::iter::Iterator;
use std::slice;
use std::fmt;

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

	pub fn chunks(&self) -> slice::Chunks<f64> {
		self.data.chunks(self.columns)
	}
}

impl Iterator for Matrix {
	type Item = Vec<f64>;

	fn next(&mut self) -> Option<Vec<f64>> {
		Some(vec![])
	}
}

impl fmt::Debug for Matrix {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::new();

		s.push_str("[");

		if self.rows != 1 {
			s.push_str("\n");
		}

		for row in self.chunks() {
			if self.rows != 1 {
				s.push_str("  ");
			}

			let mut j = 0;
			for item in row {
				j += 1;
				s.push_str(&item.to_string());
				if (j != row.len()) {
					s.push_str(" ")
				}
			}
			if self.rows != 1 {
				s.push_str("\n");
			}
		}

		s.push_str("]");

		write!(f, "{}", s)
	}
}

impl Add<f64> for Matrix {

	type Output = Matrix;

	fn add(self, rhs: f64) -> Matrix {
		let mut data = self.data.clone();

		for i in 0 .. self.data.len() {
			data[i] += rhs;
		}

		Matrix::new(self.rows, self.columns, data)
	}
}

impl Add<Matrix> for Matrix {

	type Output = Matrix;

	fn add(self, rhs: Matrix) -> Matrix {
		let mut data = self.data.clone();

		for i in 0 .. self.data.len() {
			//data[i] += rhs;
		}

		Matrix::new(self.rows, self.columns, data)
	}
}

impl Sub<f64> for Matrix {

	type Output = Matrix;

	fn sub(self, rhs: f64) -> Matrix {
		let mut data = self.data.clone();

		for i in 0 .. self.data.len() {
			data[i] -= rhs;
		}

		Matrix::new(self.rows, self.columns, data)
	}
}
