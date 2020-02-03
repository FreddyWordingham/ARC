//! Save trait.

use crate::list::Cartesian::{X, Y, Z};
use ndarray::{Array2, Array3};
use netcdf::variable::Numeric;
use serde::Serialize;
use serde_json::to_string;
use std::{fmt::Debug, fs::write, path::Path};

/// Types implementing this trait can be saved to file.
pub trait Save {
    /// Serialise the type to a given file.
    fn save(&self, path: &Path);
}

/// Serialise the type in json format.
#[inline]
pub fn as_json<T: Serialize>(instance: &T, path: &Path) {
    write(
        path,
        to_string(instance).expect("Unable to serialise object."),
    )
    .expect("Unable to write json file.");
}

impl<T: Debug + Numeric> Save for Array2<T> {
    #[inline]
    fn save(&self, path: &Path) {
        let mut file = netcdf::create(path).expect("Unable to create file.");

        let shape = self.shape();

        let dim1_name = "x";
        file.add_dimension(
            dim1_name,
            *shape.get(X as usize).expect("Invalid dimension index."),
        )
        .expect("Unable to add X dimension.");
        let dim2_name = "y";
        file.add_dimension(
            dim2_name,
            *shape.get(Y as usize).expect("Invalid dimension index."),
        )
        .expect("Unable to add Y dimension.");

        let mut var = file
            .add_variable::<T>("data", &[dim1_name, dim2_name])
            .expect("Unable to add dataslice entry.");
        var.put_values(
            self.as_slice().expect("Unable to write dataslice values."),
            None,
            None,
        )
        .expect("Unable to store dataslice values.");
    }
}

impl<T: Debug + Numeric> Save for Array3<T> {
    #[inline]
    fn save(&self, path: &Path) {
        let mut file = netcdf::create(path).expect("Unable to create file.");

        let shape = self.shape();

        let dim1_name = "x";
        file.add_dimension(
            dim1_name,
            *shape.get(X as usize).expect("Invalid dimension index."),
        )
        .expect("Unable to add X dimension.");
        let dim2_name = "y";
        file.add_dimension(
            dim2_name,
            *shape.get(Y as usize).expect("Invalid dimension index."),
        )
        .expect("Unable to add Y dimension.");
        let dim3_name = "z";
        file.add_dimension(
            dim3_name,
            *shape.get(Z as usize).expect("Invalid dimension index."),
        )
        .expect("Unable to add Z dimension.");

        let mut var = file
            .add_variable::<T>("data", &[dim1_name, dim2_name, dim3_name])
            .expect("Unable to add datacube entry.");
        var.put_values(
            self.as_slice().expect("Unable to write datacube values."),
            None,
            None,
        )
        .expect("Unable to store datacube values.");
    }
}
