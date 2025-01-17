use csv::ReaderBuilder;
// use serde::Deserializer;
use std::error::Error;
use serde::Deserialize;


use std::fs::File; //This is allows functionality with files. 

use std::io::{self, BufRead}; //Allows reading lines from a file (io input output operations)
//bufread: read chuncks of data at a time improving performance. 

fn read_file(file_path: &str) -> io::Result<()> {
    //Opens file at specific path, returns result type. This is used for error handling. <()> mean function succes or fails
    //? Will propagate any error to the caller if opening fails. 
    let file = File::open(file_path)?;
    
    // Wrap file in bufferred reader, this is efficient line by line reading
    //without loading entire file into memory
    let reader= io::BufReader::new(file);
    
    // Line by line iteration over file wrapped in a Result to handle potential errors
    for line in reader.lines() {

        //If there is an error ? will propagate it to the caller
        let line = line?;
        println!("{}", line);
    }

    //Indicates function completed successfully. 
    Ok(())
}


fn read_csv(file_path: &str) -> io::Result<()> {

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let columns: Vec<&str> = line.split(',').collect();
        //where there is a comma, split line into substrings
        //Returns and interator over the substrings 
        //.collect() This "Consumes the iterator and collects the substrings into a vector"
        //Vec<&str> This is a vector of string slices, each slice is a part of the original string
       
        println!("{:?}", columns);
        //Prints vector of columns
        //? is used for debugging 
    }
    Ok(())
}


#[derive(Debug, Deserialize)]
struct DataRow {
    id: usize,
    value: f64,
    label: String,
}

fn read_csv_to_struct(file_path: &str) -> Result<Vec<DataRow>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
    .has_headers(true)
    .from_path(file_path)?;

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: DataRow = result?;
        records.push(record);
    }
    Ok(records)
} 

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data.csv";
    let rows = read_csv_to_struct(file_path)?;

    for row in rows {
        println!("{:?}", row);
    }

    Ok(())
}