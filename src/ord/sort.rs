//! Sorting functions.

/// Function used to stick ordered sub-vectors into a single vector.
#[inline]
#[must_use]
pub fn stitch<U: Ord, T>(input: Vec<Vec<(U, Vec<T>)>>) -> Vec<T> {
    let mut flat: Vec<_> = input.into_iter().flatten().collect();
    flat.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("Ordering comparison failed."));

    flat.into_iter().flat_map(|a| a.1).collect()
}
