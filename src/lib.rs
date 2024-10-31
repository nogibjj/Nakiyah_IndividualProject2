use csv::ReaderBuilder;
use reqwest::blocking::get;
use rusqlite::{params, Connection};
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

/// Downloads data from a specified URL and saves it to a local file.
pub fn extract_data(url: &str, file_path: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;

    if response.status().is_success() {
        let mut file = File::create(file_path)?;
        file.write_all(&response.bytes()?)?;
        Ok(file_path.to_string())
    } else {
        Err(format!(
            "Failed to download file. Status code: {}",
            response.status()
        )
        .into())
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
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;
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
        conn.execute(
            insert_query,
            params![
                record[0], record[1], record[2], record[3], record[4], record[5], record[6],
                record[7], record[8],
            ],
        )?;
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
        );",
    )?;
    Ok(())
}

/// Retrieves and displays the top `n` records from the `worker_health` table.
pub fn query_data(n: u32) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("database1.db")?;
    let mut stmt = conn.prepare("SELECT * FROM worker_health LIMIT ?1")?;
    let rows = stmt.query_map([n], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, i32>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, i32>(6)?,
            row.get::<_, String>(7)?,
            row.get::<_, bool>(8)?,
        ))
    })?;

    println!("Top {} rows of worker_health table:", n);
    for row in rows {
        println!("{:?}", row?);
    }

    log_query(&format!("SELECT * FROM worker_health LIMIT {};", n))?;
    Ok(())
}

/// Queries a specific record based on `employee_id`.
pub fn query_specific_record(employee_id: i32) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("database1.db")?;
    let mut stmt = conn.prepare("SELECT * FROM worker_health WHERE Employee_ID = ?1")?;
    let mut rows = stmt.query([employee_id])?;

    if let Some(row) = rows.next()? {
        println!(
            "Record with Employee_ID {}:\n{:?}",
            employee_id,
            (
                row.get::<_, i32>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i32>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, i32>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, bool>(8)?
            )
        );
    } else {
        println!("No record found for Employee_ID {}", employee_id);
    }

    log_query(&format!(
        "SELECT * FROM worker_health WHERE Employee_ID = {};",
        employee_id
    ))?;
    Ok(())
}

/// Creates a new record in `worker_health` if the `Employee_ID` does not already exist.
pub fn create_record(
    id: i32,
    age: i32,
    jobrole: &str,
    industry: &str,
    experience: i32,
    worklocation: &str,
    hoursworked: i32,
    mhcondition: &str,
    access: bool,
) -> Result<(), Box<dyn Error>> {
    // Open a connection to the SQLite database.
    let conn = Connection::open("database1.db")?;

    // Check if the record with the given Employee_ID already exists.
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM worker_health WHERE Employee_ID = ?1)",
        [id],
        |row| row.get(0),
    )?;

    // If the record exists, print a message and exit the function.
    if exists {
        println!("Cannot create record; Employee_ID {} already exists.", id);
        return Ok(());
    }

    // Insert the new record into the worker_health table.
    conn.execute(
        "INSERT INTO worker_health (Employee_ID, Age, Job_Role, Industry, Years_of_Experience, Work_Location, Hours_Worked_Per_Week, Mental_Health_Condition, Access_to_Mental_Health_Resources) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![id, age, jobrole, industry, experience, worklocation, hoursworked, mhcondition, access],
    )?;

    // Log the SQL query for record creation.
    log_query(&format!(
        "INSERT INTO worker_health VALUES ({}, {}, '{}', '{}', {}, '{}', {}, '{}', {});",
        id, age, jobrole, industry, experience, worklocation, hoursworked, mhcondition, access
    ))?;

    // Print a success message and query the specific record to show its details.
    println!("Record with Employee_ID {} created successfully.", id);
    query_specific_record(id)?;

    Ok(())
}

/// Updates an existing record in `worker_health` if the `Employee_ID` exists.
pub fn update_record(
    id: i32,
    age: i32,
    jobrole: &str,
    industry: &str,
    experience: i32,
    worklocation: &str,
    hoursworked: i32,
    mhcondition: &str,
    access: bool,
) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("database1.db")?;

    // Check if the record with the given Employee_ID exists.
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM worker_health WHERE Employee_ID = ?1)",
        [id],
        |row| row.get(0),
    )?;

    // If the record does not exist, print a message and exit the function.
    if !exists {
        println!("Cannot update record; Employee_ID {} does not exist.", id);
        return Ok(());
    }

    // Update the existing record in the worker_health table.
    conn.execute(
        "UPDATE worker_health SET 
         Age = ?2, Job_Role = ?3, Industry = ?4, 
         Years_of_Experience = ?5, Work_Location = ?6, 
         Hours_Worked_Per_Week = ?7, Mental_Health_Condition = ?8, 
         Access_to_Mental_Health_Resources = ?9 
         WHERE Employee_ID = ?1",
        params![
            id,
            age,
            jobrole,
            industry,
            experience,
            worklocation,
            hoursworked,
            mhcondition,
            access
        ],
    )?;

    // Log the SQL query for record update.
    log_query(&format!(
        "UPDATE worker_health SET 
         Age = {}, Job_Role = '{}', Industry = '{}', 
         Years_of_Experience = {}, Work_Location = '{}', 
         Hours_Worked_Per_Week = {}, Mental_Health_Condition = '{}', 
         Access_to_Mental_Health_Resources = {} 
         WHERE Employee_ID = {};",
        age, jobrole, industry, experience, worklocation, hoursworked, mhcondition, access, id
    ))?;

    // Print a success message.
    println!("Record with Employee_ID {} updated successfully.", id);
    Ok(())
}

/// Deletes a record in `worker_health` based on `Employee_ID`.
pub fn delete_record(employee_id: i32) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("database1.db")?;

    // Check if the record with the given Employee_ID exists.
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM worker_health WHERE Employee_ID = ?1)",
        [employee_id],
        |row| row.get(0),
    )?;

    // If the record does not exist, print a message and exit the function.
    if !exists {
        println!(
            "Cannot delete record; Employee_ID {} does not exist.",
            employee_id
        );
        return Ok(());
    }

    // Delete the record from the worker_health table.
    conn.execute(
        "DELETE FROM worker_health WHERE Employee_ID = ?1",
        [employee_id],
    )?;

    // Log the SQL query for record deletion.
    log_query(&format!(
        "DELETE FROM worker_health WHERE Employee_ID = {};",
        employee_id
    ))?;

    // Print a success message.
    println!(
        "Record with Employee_ID {} deleted successfully.",
        employee_id
    );
    Ok(())
}
