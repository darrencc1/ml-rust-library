use csv::ReaderBuilder;
// use serde::Deserializer;
use std::error::Error;
use serde::Deserialize;
use std::collections::HashMap;

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
//This struct is to map the columns of a csv file, ("Package related fields together")
//**Structs: Blueprints for creating and oprganizing data in a structured way.**/
//****May need to generalize this using a HashMap for more dynamic row parsing for varying column structures.*****/
//****This Struct will only work if the csv has 3 columns (Not very useful for machine learning. (Change to Dynamic Handling with HashMap!)) ****/
struct DataRow {
    let mut map = HashMap::new();//Createsw and empty hash map
    
    // id: usize,
    // value: f64,
    // label: String,
}


fn read_csv_to_struct(file_path: &str) -> Result<Vec<DataRow>, Box<dyn Error>> {
    //Parses CSV into a vaector of DataRow structs
    let mut rdr = ReaderBuilder::new()
    .has_headers(true)
    //Skips header row in csv automatically. 
    .from_path(file_path)?;

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        //deserialize heach row into a DataRow instance (Struct above)
        
        let record: DataRow = result?;
        //Will show errors if ishues

        records.push(record);
        //This collects/records deserialized rows into a vector
    }
    Ok(records)
} 

// fn main() -> Result<(), Box<dyn Error>> {
//     let file_path = "data.csv";
//     let rows = read_csv_to_struct(file_path)?;

//     for row in rows {
//         println!("{:?}", row);
//     }

//     Ok(())
// }

pub fn run_csv_demo(file_path: &str) -> Result<Vec<DataRow>, Box<dyn Error>> {
    let data = read_csv_to_struct(file_path)?;
    for row in &data {
        println!("{:?}", row);
    }
    Ok(data)
}
