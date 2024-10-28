use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use csv::ReaderBuilder;
use rusqlite::{params, Connection};
use std::fs::OpenOptions;

/// Downloads data from a specified URL and saves it to a local file.
pub fn extract_data(url: &str, file_path: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;

    if response.status().is_success() {
        let mut file = File::create(file_path)?;
        file.write_all(&response.bytes()?)?;
        Ok(file_path.to_string())
    } else {
        Err(format!("Failed to download file. Status code: {}", response.status()).into())
    }
}

/// Logs an SQL query to `queryLog.md` in markdown format.
pub fn log_query(query: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("queryLog.md")?;
    writeln!(file, "```sql\n{}\n```\n", query)?;
    Ok(())
}

/// Cleans and prepares data from a CSV file by converting specific columns to appropriate data types.
pub fn clean_data(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut cleaned_data = Vec::new();

    for result in reader.records() {
        let record = result?;
        let employee_id = record.get(0).unwrap_or("").to_string();
        let age = record.get(1).unwrap_or("").to_string();
        let job_role = record.get(2).unwrap_or("").to_string();
        let industry = record.get(3).unwrap_or("").to_string();
        let years_of_experience = record.get(4).unwrap_or("").to_string();
        let work_location = record.get(5).unwrap_or("").to_string();
        let hours_worked_per_week = record.get(6).unwrap_or("").to_string();
        let mental_health_condition = record.get(7).unwrap_or("").to_string();
        let access_to_resources = record.get(8).unwrap_or("").to_string();

        if let (Ok(age), Ok(years_of_experience), Ok(hours_worked)) = (
            age.parse::<i32>(),
            years_of_experience.parse::<i32>(),
            hours_worked_per_week.parse::<i32>(),
        ) {
            cleaned_data.push(vec![
                employee_id,
                age.to_string(),
                job_role,
                industry,
                years_of_experience.to_string(),
                work_location,
                hours_worked.to_string(),
                mental_health_condition,
                (access_to_resources == "Yes").to_string(),
            ]);
        }
    }
    Ok(cleaned_data)
}

/// Creates the `worker_health` table in the SQLite database and loads cleaned data into it.
pub fn load_data(data: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("database1.db")?;

    conn.execute("DROP TABLE IF EXISTS worker_health", [])?;
    conn.execute(
        "CREATE TABLE worker_health (
            Employee_ID INTEGER PRIMARY KEY,
            Age INTEGER,
            Job_Role TEXT,
            Industry TEXT,
            Years_of_Experience INTEGER,
            Work_Location TEXT,
            Hours_Worked_Per_Week INTEGER,
            Mental_Health_Condition TEXT,
            Access_to_Mental_Health_Resources BOOLEAN
        )",
        [],
    )?;

    let insert_query = "INSERT INTO worker_health VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
    for record in data {
        conn.execute(insert_query, params![
            record[0], record[1], record[2], record[3],
            record[4], record[5], record[6], record[7],
            record[8],
        ])?;
    }
    
    log_query(
        "CREATE TABLE worker_health (
            Employee_ID INTEGER PRIMARY KEY,
            Age INTEGER,
            Job_Role TEXT,
            Industry TEXT,
            Years_of_Experience INTEGER,
            Work_Location TEXT,
            Hours_Worked_Per_Week INTEGER,
            Mental_Health_Condition TEXT,
            Access_to_Mental_Health_Resources BOOLEAN
        );"
    )?;
    Ok(())
}
