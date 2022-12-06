use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::io;

fn bogosort<T: Ord+ std::fmt::Debug>(arr: &mut [T]) {
    loop {
        // Shuffle the array
        arr.shuffle(&mut rand::thread_rng());

        // Dont stop shuffling until the array is sorted.
        let mut is_sorted = true;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                is_sorted = false;
                break;
            }
        }

        if is_sorted {
            break;
        }
        // Print the progress so far
        println!("{:?}", arr);
    }
}

fn main() {
    // Request user input
    println!("Enter an amount of numbers to sort: ");

    // Get user input
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let mut n: i32 = n.trim().parse().expect("invalid input");
    n = n + 1;

    // Create the randomised array and shuffle it so it isn't instantly solved
    let mut arr: Vec<i32> = (1..n).collect();
    arr.shuffle(&mut thread_rng());

    // Run the bogosort function
    bogosort(&mut arr);

    // Print the sorted array
    println!("{:?}", arr);
}