use matrix::Matrix;

#[test]
#[should_panic(expected = "assertion failed")]
fn matrix_new() {
	Matrix::new(2, 2, vec![]);
}

#[test]
fn matrix_new2() {
	let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

	let (rows, columns) = m.shape();
	assert!(2 == rows);
	assert!(2 == columns);

	m.print();

	let (rows, columns) = m.shape();
	assert!(2 == rows);
	assert!(2 == columns);

	assert!(4 == m.size());
}

#[test]
fn matrix_add() {
	let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
	let b = m + 3.0;
	b.print()
}
