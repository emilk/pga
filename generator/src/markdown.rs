use std::io::Write;

use itertools::{izip, Itertools};

pub fn table(headers: impl IntoIterator<Item = String>, rows: impl IntoIterator<Item = Vec<String>>) -> String {
	let headers: Vec<String> = headers.into_iter().collect();
	let rows: Vec<Vec<String>> = rows.into_iter().collect();

	let mut col_widths: Vec<usize> = headers.iter().map(String::len).collect();
	for row in &rows {
		assert_eq!(row.len(), col_widths.len());
		for (col_idx, cell) in row.iter().enumerate() {
			col_widths[col_idx] = col_widths[col_idx].max(cell.len());
		}
	}

	let mut s = vec![];
	write!(
		&mut s,
		"| {} |\n",
		izip!(&headers, &col_widths)
			.map(|(header, width)| format!("{:<w$}", header, w = width))
			.format(" | ")
	)
	.unwrap();
	write!(
		&mut s,
		"| {} |\n",
		col_widths
			.iter()
			.map(|width| format!("{:-<w$}", "", w = width))
			.format(" | ")
	)
	.unwrap();
	for row in &rows {
		write!(
			&mut s,
			"| {} |\n",
			izip!(row, &col_widths)
				.map(|(cell, width)| format!("{:<w$}", cell, w = width))
				.format(" | ")
		)
		.unwrap();
	}
	String::from_utf8(s).unwrap()
}
