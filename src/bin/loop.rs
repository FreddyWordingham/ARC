use arc::util::ParProgressBar;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn main() {
    println!("beep");

    let total = 1_000_000_000;
    let num_threads = 4;

    let pb = ParProgressBar::new("Building", total as u64);
    let pb = Arc::new(Mutex::new(pb));

    let thread_ids: Vec<usize> = (0..num_threads).collect();

    let now = std::time::Instant::now();
    let thread_data: Vec<_> = thread_ids
        .par_iter()
        .map(|id| task(*id, Arc::clone(&pb), total / 1000))
        .collect();
    pb.lock().unwrap().finish_with_message("Complete.");
    println!("Took {} sec to build.", now.elapsed().as_secs());

    let now = std::time::Instant::now();
    let data: Vec<usize> = stitch(thread_data);
    println!("Took {} sec to stitch.", now.elapsed().as_secs());

    println!("Data len: {}", data.len());

    for (i, d) in data.iter().enumerate() {
        assert!(i == *d);
    }

    println!("Success!");
}

fn task(_id: usize, pb: Arc<Mutex<ParProgressBar>>, block_size: u64) -> Vec<(usize, Vec<usize>)> {
    let mut blocks = Vec::new();
    while let Some((start, end)) = {
        let mut pb = pb.lock().unwrap();
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        // println!("{}: Running from {} -> {}", id, start, end);

        let mut data = Vec::with_capacity((end - start) as usize);
        for n in start..end {
            data.push(n as usize);
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
