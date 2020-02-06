use arc::util::ParProgressBar;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn main() {
    println!("beep");

    let shape = [40, 30, 50];
    let total = shape[0] * shape[1] * shape[2];
    let num_threads = 1;

    let pb = ParProgressBar::new("Building", total as u64);
    let pb = Arc::new(Mutex::new(pb));

    let thread_ids: Vec<usize> = (0..num_threads).collect();

    let now = std::time::Instant::now();
    let thread_data: Vec<_> = thread_ids
        .par_iter()
        .map(|id| task(*id, Arc::clone(&pb), 1000, shape))
        .collect();
    pb.lock().unwrap().finish_with_message("Complete.");
    println!("Took {} sec to build.", now.elapsed().as_secs());

    let now = std::time::Instant::now();
    let data: Vec<[usize; 3]> = stitch(thread_data);
    println!("Took {} sec to stitch.", now.elapsed().as_secs());

    println!("Data len: {}", data.len());

    let arr3 = ndarray::Array3::from_shape_vec(shape, data).unwrap();

    for xi in 0..shape[0] {
        for yi in 0..shape[1] {
            for zi in 0..shape[2] {
                let id = [xi, yi, zi];
                let a = arr3[id];

                assert!(id == a, "{:?} -> {:?}", id, a);
            }
        }
    }

    println!("Success!");
}

fn task(
    _id: usize,
    pb: Arc<Mutex<ParProgressBar>>,
    block_size: u64,
    shape: [usize; 3],
) -> Vec<(usize, Vec<[usize; 3]>)> {
    let mut blocks = Vec::new();
    while let Some((start, end)) = {
        let mut pb = pb.lock().unwrap();
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        // println!("{}: Running from {} -> {}", id, start, end);

        let mut data: Vec<[usize; 3]> = Vec::with_capacity((end - start) as usize);
        for n in start..end {
            data.push(arc::math::three_dim(n as usize, shape));
        }
        blocks.push((start as usize, data));
    }

    blocks
}

fn stitch<U: Ord, T>(input: Vec<Vec<(U, Vec<T>)>>) -> Vec<T> {
    let mut flat: Vec<_> = input.into_iter().flatten().collect();
    flat.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    flat.into_iter().map(|a| a.1).flatten().collect()
}
