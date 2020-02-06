//! Sorting functions.

/// Function used to stick ordered sub-vectors into a single vector.
#[inline]
#[must_use]
pub fn stitch<U: Ord, T>(input: Vec<Vec<(U, Vec<T>)>>) -> Vec<T> {
    let mut flat: Vec<_> = input.into_iter().flatten().collect();
    flat.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    flat.into_iter().map(|a| a.1).flatten().collect()
}
