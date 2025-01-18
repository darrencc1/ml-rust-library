#[cfg(test)]
//This ensures that this will ONLY run when running tests and not in actual building / production

mod tests {
    use super::*;
    //super::*; importa ALL public items from parent model (ex. read_csv_to_struct)
    
    use std::fs::File;
    use std::io::Write;
    //Create and write files for testing. 

    fn create_test_csv(file_path: &str) {
        let mut file = File::create(file_path).expect("Unable to create test CSV file");
        //This creates a file for writing, error happens if there are permission issues(not sure of other issues for this)
        writeln!(file, "id,value,label").unwrap();
        writeln!(file, "1,10.5,Positive").unwrap();
        writeln!(file, "2,5.0,Negative").unwrap();
        writeln!(file, "3,7.8,Neutral").unwrap();
        //.unwrap() will cause a "panic" id there are errors, making debugging easier. 
    }

    #[test]
    fn test_read_csv_to_struct() {
        let test_file = "test_data.csv";
        create_test_csv(test_file);

        let result = read_csv_to_struct(test_file);
        let _ = std::fs::remove_file(test_file);
        //This deletes the test_file after the test runs 

        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 3);
        //This Verigies that EXACTLY 3 rows were parsed (3 rows written in create_test_csv)

        assert_eq!(data[0].id, 1);
        //Checks id of first row ensuring it is correct
        assert_eq!(data[0].value, 10.5);
        assert_eq!(data[0].label, "Positive");
        //Confirms 1st row has correct labels and strings. 
    }
}
//***************Things to Do ****************/
//Empy file test case
//Improper row formats or missing headers
//edge cases such as massive numbers, 0, - values etc.
