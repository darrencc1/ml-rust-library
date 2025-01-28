#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module
    use std::fs::File;
    use std::io::Write;
    

    /// Creates a test CSV file for testing purposes
    fn create_test_csv(file_path: &str) {
        let mut file = File::create(file_path).expect("Unable to create test CSV file");
        writeln!(file, "id,value,label").unwrap(); // Write headers
        writeln!(file, "1,10.5,Positive").unwrap(); // First row
        writeln!(file, "2,5.0,Negative").unwrap(); // Second row
        writeln!(file, "3,7.8,Neutral").unwrap(); // Third row
    }

    /// Test for the `read_csv_to_hashmap` function
    #[test]
    fn test_read_csv_to_hashmap() {
        let test_file = "test_data.csv";
        create_test_csv(test_file); // Create the test CSV

        let result = read_csv_to_hashmap(test_file); // Call the function being tested
        let _ = std::fs::remove_file(test_file); // Clean up: remove the test file after test runs

        // Ensure the result is Ok (no errors)
        assert!(result.is_ok());
        let data = result.unwrap();

        // Check that the correct number of rows was parsed
        assert_eq!(data.len(), 3); // Expecting 3 rows

        // Validate the contents of the first row
        assert_eq!(data[0].get("id").unwrap(), "1");
        assert_eq!(data[0].get("value").unwrap(), "10.5");
        assert_eq!(data[0].get("label").unwrap(), "Positive");

        // Validate the second row
        assert_eq!(data[1].get("id").unwrap(), "2");
        assert_eq!(data[1].get("value").unwrap(), "5.0");
        assert_eq!(data[1].get("label").unwrap(), "Negative");

        // Validate the third row
        assert_eq!(data[2].get("id").unwrap(), "3");
        assert_eq!(data[2].get("value").unwrap(), "7.8");
        assert_eq!(data[2].get("label").unwrap(), "Neutral");
    }

    /// Additional test case: Handling an empty file
    #[test]
    fn test_empty_csv() {
        let test_file = "empty_test.csv";

        // Create an empty file
        let _ = File::create(test_file).expect("Unable to create empty test CSV file");

        let result = read_csv_to_hashmap(test_file); // Call the function
        let _ = std::fs::remove_file(test_file); // Clean up: remove the test file

        // Ensure the result is Ok and returns an empty vector
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0); // Expecting 0 rows
    }

    /// Additional test case: Missing headers
    #[test]
    fn test_missing_headers() {
        let test_file = "missing_headers_test.csv";

        // Create a file with no headers
        let mut file = File::create(test_file).expect("Unable to create missing headers test CSV file");
        writeln!(file, "1,10.5,Positive").unwrap();
        writeln!(file, "2,5.0,Negative").unwrap();
        writeln!(file, "3,7.8,Neutral").unwrap();

        let result = read_csv_to_hashmap(test_file); // Call the function
        let _ = std::fs::remove_file(test_file); // Clean up: remove the test file

        // Ensure the result is an error (missing headers should fail)
        assert!(result.is_err());
    }

    /// Additional test case: Large numbers and edge cases
    #[test]
    fn test_edge_cases() {
        let test_file = "edge_cases_test.csv";

        // Create a file with edge case data
        let mut file = File::create(test_file).expect("Unable to create edge cases test CSV file");
        writeln!(file, "id,value,label").unwrap(); // Write headers
        writeln!(file, "1,0,Zero").unwrap();
        writeln!(file, "2,-5.0,Negative").unwrap();
        writeln!(file, "3,1000000000,Large").unwrap();

        let result = read_csv_to_hashmap(test_file); // Call the function
        let _ = std::fs::remove_file(test_file); // Clean up: remove the test file

        // Ensure the result is Ok
        assert!(result.is_ok());
        let data = result.unwrap();

        // Validate the contents of the edge case rows
        assert_eq!(data[0].get("value").unwrap(), "0");
        assert_eq!(data[1].get("value").unwrap(), "-5.0");
        assert_eq!(data[2].get("value").unwrap(), "1000000000");
    }
}

fn create_large_test_csv(file_path: &str, num_rows: usize) {
    let mut file = File::create(file_path).expect("Unable to create test CSV file");
    writeln!(file, "id,value,label").unwrap(); // Write headers
    for i in 1..=num_rows {
        writeln!(file, "{},{},Row{}", i, i as f64 * 1.1, i).unwrap();
    }
}

/// Test to validate batch processing and multi-threading
#[test]
fn test_read_csv_in_batches_multithreading() {
    let test_file = "large_test_data.csv";
    let total_rows = 1050; // Total rows in the CSV
    let batch_size = 100; // Adjusted batch size

    // Create a test CSV file with 1050 rows
    create_large_test_csv(test_file, total_rows);

    // Create shared data structure to track processed rows
    let processed_rows = Arc::new(Mutex::new(Vec::new()));

    // Create thread handles
    let mut handles = vec![];

    // Simulate batch processing with threads
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(test_file)
        .expect("Unable to read test CSV file");

    let headers: Vec<String> = rdr.headers().unwrap().iter().map(String::from).collect();

    // Batch buffer
    let mut batch = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let mut row_map = HashMap::new();

        for (header, value) in headers.iter().zip(record.iter()) {
            row_map.insert(header.clone(), value.to_string());
        }

        batch.push(row_map);

        if batch.len() == batch_size {
            let processed_rows_clone = Arc::clone(&processed_rows);
            let current_batch = batch.clone();

            handles.push(std::thread::spawn(move || {
                let processed_batch: Vec<_> = current_batch
                    .into_iter()
                    .map(|mut row| {
                        row.insert("processed".to_string(), "true".to_string());
                        row
                    })
                    .collect();

                let mut shared = processed_rows_clone.lock().unwrap();
                shared.extend(processed_batch);
            }));

            batch.clear();
        }
    }

    // Process remaining rows
    if !batch.is_empty() {
        let processed_rows_clone = Arc::clone(&processed_rows);
        let remaining_batch = batch.clone();

        handles.push(std::thread::spawn(move || {
            let processed_batch: Vec<_> = remaining_batch
                .into_iter()
                .map(|mut row| {
                    row.insert("processed".to_string(), "true".to_string());
                    row
                })
                .collect();

            let mut shared = processed_rows_clone.lock().unwrap();
            shared.extend(processed_batch);
        }));
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Validate results
    let final_data = processed_rows.lock().unwrap();
    assert_eq!(final_data.len(), total_rows); // Ensure all rows were processed
    for (i, row) in final_data.iter().enumerate() {
        assert_eq!(row.get("id").unwrap(), &(i + 1).to_string());
        assert_eq!(row.get("processed").unwrap(), "true");
    }

    // Clean up test file
    let _ = std::fs::remove_file(test_file);
}

/// Test to validate the `read_csv_in_batches` function
#[test]
fn test_read_csv_in_batches() {
    let test_file = "batch_test_data.csv";
    let total_rows = 1050; // Total rows in the CSV
    let batch_size = 100; // Set the batch size

    // Create a test CSV file with 1050 rows
    create_large_test_csv(test_file, total_rows);

    // Initialize a counter for rows processed in batches
    let mut total_processed_rows = 0;

    // Call the `read_csv_in_batches` function with a closure to simulate batch processing
    let result = read_csv_in_batches(test_file, batch_size, |batch| {
        // Check batch size
        if total_processed_rows + batch.len() < total_rows {
            assert_eq!(batch.len(), batch_size); // All full batches should have the correct size
        }
        
        // Simulate processing by counting rows
        total_processed_rows += batch.len();
    });

    // Clean up test file
    let _ = std::fs::remove_file(test_file);

    // Ensure the function completed successfully
    assert!(result.is_ok());

    // Validate that all rows were processed
    assert_eq!(total_processed_rows, total_rows);
}
