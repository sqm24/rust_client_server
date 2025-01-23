use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::fs::File;
use std::io::{self, Write};

fn process_file(filename: &str) -> io::Result<String> {
    // Pretend we're processing a file
    Ok(format!("Processed: {}", filename))
}

fn main() {
    // Create a channel to send results back to the main thread
    let (tx, rx) = mpsc::channel();

    // List of filenames to process
    let filenames = vec!["file1.txt", "file2.txt", "file3.txt"];

    // Create a thread pool to process the files
    let mut handles = vec![];

    for filename in filenames {
        let tx = tx.clone(); // Clone the sender to send to the same channel
        let filename = filename.to_string();

        let handle = thread::spawn(move || {
            // Process the file
            match process_file(&filename) {
                Ok(result) => tx.send(result).unwrap(),
                Err(e) => tx.send(format!("Error: {}", e)).unwrap(),
            }
        });
        handles.push(handle);
    }

    // Wait for the threads to complete and collect the results
    for _ in 0..3 {
        let result = rx.recv().unwrap();
        println!("{}", result); // Output the result of file processing
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
