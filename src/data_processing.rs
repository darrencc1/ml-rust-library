use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

const BATCH_SIZE: usize = 1000; // Define the size of each batch

/// Function to read a CSV file and process rows in batches using threads
fn read_csv_in_batches(file_path: &str) -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    // Get the headers (column names)
    let headers: Vec<String> = rdr.headers()?.iter().map(String::from).collect();

    // Shared vector to store processed rows
    let processed_data = Arc::new(Mutex::new(Vec::new()));

    // Vector to hold thread handles
    let mut handles = vec![];

    // Batch buffer
    let mut batch = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut row_map = HashMap::new();

        // Map the headers and row values
        for (header, value) in headers.iter().zip(record.iter()) {
            row_map.insert(header.clone(), value.to_string());
        }

        // Add the row to the batch
        batch.push(row_map);

        // When the batch reaches the defined size, process it in a thread
        if batch.len() == BATCH_SIZE {
            // Clone the shared vector for the thread
            let processed_data_clone = Arc::clone(&processed_data);

            // Move the batch into a new thread
            let current_batch = batch.clone();
            handles.push(thread::spawn(move || {
                // Simulate batch processing (e.g., normalization, encoding)
                let processed_batch = current_batch
                    .into_iter()
                    .map(|mut row| {
                        // Example: Add a new column "processed" for demonstration
                        row.insert("processed".to_string(), "true".to_string());
                        row
                    })
                    .collect::<Vec<_>>();

                // Save the processed batch to the shared vector
                let mut data = processed_data_clone.lock().unwrap();
                data.extend(processed_batch);
            }));

            // Clear the batch for the next set of rows
            batch.clear();
        }
    }

    // Process any remaining rows in the batch
    if !batch.is_empty() {
        let processed_data_clone = Arc::clone(&processed_data);
        handles.push(thread::spawn(move || {
            let processed_batch = batch
                .into_iter()
                .map(|mut row| {
                    row.insert("processed".to_string(), "true".to_string());
                    row
                })
                .collect::<Vec<_>>();

            let mut data = processed_data_clone.lock().unwrap();
            data.extend(processed_batch);
        }));
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Access the processed data
    let final_data = processed_data.lock().unwrap();
    println!("Processed {} rows.", final_data.len());
    for (i, row) in final_data.iter().take(5).enumerate() {
        println!("Row {}: {:?}", i + 1, row); // Print the first 5 rows
    }

    Ok(())
}

#[cfg(test)]
/// Main function to test batch processing with threads
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "large_dataset.csv"; // Replace with your dataset
    read_csv_in_batches(file_path)?;
    Ok(())
}
