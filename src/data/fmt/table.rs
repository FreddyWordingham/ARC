//! Table implementation.

use crate::{access, file::Save};
use ndarray::{Array2, ArrayView1};
use std::{
    fmt::{Debug, Display, Formatter, Result},
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

/// Table structure for writing csv files.
#[derive(Debug)]
pub struct Table<T> {
    /// Column headings.
    headings: Vec<String>,
    /// Data.
    data: Array2<T>,
}

impl<T: Clone> Table<T> {
    access!(headings, Vec<String>);
    access!(data, Array2<T>);

    /// Construct a new table.
    #[inline]
    #[must_use]
    pub fn new(headings: Vec<String>, data: Array2<T>) -> Self {
        Self { headings, data }
    }

    /// Construct a new table from a nested vector .
    #[inline]
    #[must_use]
    pub fn from_nested(headings: Vec<String>, data: &[Vec<T>]) -> Self {
        let num_cols = headings.len();
        let num_rows = data.len();

        Self {
            headings,
            data: Array2::from_shape_vec(
                (num_rows, num_cols),
                data.iter().flatten().cloned().collect(),
            )
            .expect("Invalid data to form table."),
        }
    }

    /// Get an array view of the requested column.
    #[inline]
    #[must_use]
    pub fn col(&self, name: &str) -> ArrayView1<T> {
        for (index, heading) in self.headings.iter().enumerate() {
            if heading == name {
                return self.data.column(index);
            }
        }

        panic!("Invalid column heading.");
    }
}

impl<T: Display> Display for Table<T> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "{:>32}",
            self.headings.first().expect("No header data to write.")
        )
        .expect("Failed to write to csv file.");
        for heading in self.headings.iter().skip(1) {
            write!(fmt, ",{:>32}", heading).expect("Failed to write to formatter.");
        }
        writeln!(fmt).expect("Failed to write to formatter.");

        for row in self.data.genrows() {
            write!(fmt, "{:>32}", row.first().expect("No row data to write."))
                .expect("Failed to write to formatter.");
            for elem in row.iter().skip(1) {
                write!(fmt, ",{:>32}", elem).expect("Failed to write to formatter.");
            }
            writeln!(fmt).expect("Failed to write to formatter.");
        }

        Ok(())
    }
}

impl<T: Debug + Display> Save for Table<T> {
    #[inline]
    fn save(&self, path: &Path) {
        let mut file =
            BufWriter::new(File::create(path).expect("Unable to create output csv file."));
        write!(file, "{}", self).expect("Failed to write to csv file.");
    }
}
