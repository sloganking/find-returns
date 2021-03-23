// use std::env;
use std::{fs, sync::mpsc, time::SystemTime};
use threadpool::ThreadPool;

fn find_returns_in(slice: &str) -> Vec<usize>{
    let mut returns = Vec::new();
    for (i, c) in slice.chars().enumerate() {
        if c == '\n'{
            returns.push(i);
        }
    }
    returns
}

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);
    // println!("{:?}", slices);

    // multi threaded analasys

        // divide string into slices for multi threading
            let slice_size = contents.len() / num_cpus::get();
            let slices = stringr::splitn(&contents, slice_size);

        let (results_tx, results_rx) = mpsc::channel();
        let pool = ThreadPool::new(num_cpus::get());
        let start_time = SystemTime::now();
        for slice in slices{
            let results_tx1 = results_tx.clone();
            pool.execute(move ||{
                let returns = find_returns_in(&slice);
                results_tx1.send(returns).unwrap();
            });
        }
        drop(results_tx);
        for received in results_rx{
            // println!("Return indexes: {:?}",received);
        }

        // print time took for completion
            let end_time = SystemTime::now();
            let duration = end_time
            .duration_since(start_time)
            .expect("Time went backwards");
            println!("Multi threaded: {:?}", duration);

    // single threaded analasys
        let start_time = SystemTime::now();

        // println!("{:?}",);
        find_returns_in(&contents);

        // print time took for completion
            let end_time = SystemTime::now();
            let duration = end_time
            .duration_since(start_time)
            .expect("Time went backwards");
            println!("Single threaded: {:?}", duration);

}