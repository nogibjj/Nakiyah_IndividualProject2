use project2::{clean_data, load_data};
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

    Ok(())
}