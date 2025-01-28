use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
//May be better to use rayon or tokio for better parallelism.
const BATCH_SIZE: usize = 1000;

/// Reads a CSV file and divides it into batches of rows.
fn read_csv(file_path: &str) -> Result<(Vec<String>, Vec<Vec<HashMap<String, String>>>), Box<dyn Error>> {
    // Open the CSV file
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    // Get headers
    let headers: Vec<String> = rdr.headers()?.iter().map(String::from).collect();

    // Read rows into batches
    let mut batches = Vec::new();
    let mut batch = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut row_map = HashMap::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            row_map.insert(header.clone(), value.to_string());
        }
        batch.push(row_map);

        if batch.len() == BATCH_SIZE {
            batches.push(batch.clone());
            batch.clear();
        }
    }

    // Push the remaining rows as the last batch
    if !batch.is_empty() {
        batches.push(batch);
    }

    Ok((headers, batches))
}
//******This may be best spot for preprocessing tasks like normalization, one-hot encoding. ******/
/// Processes a batch of rows in a separate thread.
fn process_batch(
    batch: Vec<HashMap<String, String>>,
    processed_data: Arc<Mutex<Vec<HashMap<String, String>>>>,
) {
    let processed_batch = batch
        .into_iter()
        .map(|mut row| {
            row.insert("processed".to_string(), "true".to_string());
            row
        })
        .collect::<Vec<_>>();

    let mut data = processed_data.lock().unwrap();
    data.extend(processed_batch);
}

/// Reads a CSV file and processes rows in batches using threads.
fn read_csv_in_batches(file_path: &str) -> Result<(), Box<dyn Error>> {
    let (_, batches) = read_csv(file_path)?;

    // Shared vector to store processed rows
    let processed_data = Arc::new(Mutex::new(Vec::new()));

    // Vector to hold thread handles
    let mut handles = vec![];

    for (i, batch) in batches.into_iter().enumerate() {
        let processed_data_clone = Arc::clone(&processed_data);

        handles.push(thread::spawn(move || {
            println!("Processing batch {} with {} rows...", i + 1, batch.len());
            process_batch(batch, processed_data_clone);
        }));
    }

    // Wait for all threads to complete
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("A thread panicked: {:?}", e);
        }
    }

    // Access the processed data
    let final_data = processed_data.lock().unwrap();
    println!("Processed {} rows in total.", final_data.len());
    for (i, row) in final_data.iter().take(5).enumerate() {
        println!("Row {}: {:?}", i + 1, row); // Print the first 5 rows
    }

    Ok(())
}

/// Main function to test batch processing with threads
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "large_dataset.csv"; // Replace with your dataset
    read_csv_in_batches(file_path)?;
    Ok(())
}
