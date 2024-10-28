use project2::{clean_data, load_data, create_record, delete_record, query_data, query_specific_record, update_record};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Data/Impact_of_Remote_Work_on_Mental_Health.csv";
    
    // Extract, clean, and load data
    if let Ok(cleaned_data) = clean_data(file_path) {
        load_data(cleaned_data)?;
        println!("Data has been successfully cleaned and loaded.");
    } else {
        eprintln!("Failed to clean data.");
    }

    // Query top 20 records
    query_data(20)?;

    // Query specific record
    query_specific_record(101)?;

    // Create a new record
    create_record(101, 28, "Data Scientist", "Tech", 5, "Remote", 40, "None", true)?;

    // Update a record
    update_record(101, 30, "Senior Data Scientist", "Tech", 6, "On-site", 45, "None", false)?;

    // Delete a record
    delete_record(101)?;

    Ok(())
}