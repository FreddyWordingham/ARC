//! Index producing functions.

use crate::list::Cartesian::{Y, Z};

/// Create the next three-dimensional index from the given linear index.
#[inline]
#[must_use]
pub fn three_dim(n: usize, res: [usize; 3]) -> [usize; 3] {
    let zi = n % res.get(Z as usize).expect("Missing resolution index.");
    let yi = (n / res.get(Z as usize).expect("Missing resolution index."))
        % res.get(Y as usize).expect("Missing resolution index.");
    let xi = n
        / (res.get(Y as usize).expect("Missing resolution index.")
            * res.get(Z as usize).expect("Missing resolution index."));

    [xi, yi, zi]
}
