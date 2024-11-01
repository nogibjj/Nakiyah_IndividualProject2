use project2::{
    extract_data, clean_data, create_record, delete_record, load_data, query_data, query_specific_record,
    update_record,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let url = "https://raw.githubusercontent.com/viraterletska/Impact_of_Remote_Work_on_Mental_Health/main/data/Impact_of_Remote_Work_on_Mental_Health.csv";
    let file_path = "Data/Impact_of_Remote_Work_on_Mental_Health.csv";

    // Extract data from URL and save to file
    match extract_data(url, file_path) {
        Ok(downloaded_file_path) => {
            println!("Data has been successfully downloaded to {}.", downloaded_file_path);
        }
        Err(e) => {
            eprintln!("Failed to download data: {}", e);
            return Err(e);
        }
    }

    let file_path = "Data/Impact_of_Remote_Work_on_Mental_Health.csv";

    // Extract, clean, and load data
    match clean_data(file_path) {
        Ok(cleaned_data) => {
            load_data(cleaned_data)?;
            println!("Data has been successfully cleaned and loaded.");
        }
        Err(e) => {
            eprintln!("Failed to clean data: {}", e);
            return Err(e);
        }
    }

    // Query top 20 records
    query_data(20)?;

    // Query specific record
    query_specific_record(101)?;

    // Create a new record
    create_record(
        101,
        28,
        "Data Scientist",
        "Tech",
        5,
        "Remote",
        40,
        "None",
        true,
    )?;

    // Update a record
    update_record(
        101,
        30,
        "Senior Data Scientist",
        "Tech",
        6,
        "On-site",
        45,
        "None",
        false,
    )?;

    // Delete a record
    delete_record(101)?;

    Ok(())
}
