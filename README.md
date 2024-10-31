# Nakiyah_IndividualProject2

[![Rust CI/CD Pipeline](https://github.com/nogibjj/Nakiyah_IndividualProject2/actions/workflows/rust.yml/badge.svg)](https://github.com/nogibjj/Nakiyah_IndividualProject2/actions/workflows/rust.yml)

## Purpose of this project

## Project Overview
# ETL Pipeline Project

This project implements an ETL (Extract, Transform, Load) pipeline using Rust. It processes data from external public datasets and stores it in a SQLite database. The key stages of the pipeline are as follows:

## Extract

- Data is fetched from a public GitHub repository and loaded into a local CSV file.

## Transform & Load

- The CSV file is cleaned and pre-processed to ensure the data is structured correctly and ready for analysis.
- The cleaned data is then loaded into a SQLite `.db` file, where it can be efficiently queried for further analysis.

## Querying

- SQL queries are verified to ensure they return the expected results, such as retrieving the top 20 rows from a specific table.

## Testing

- A suite of unit tests is executed using Python's subprocess module to simulate the full pipeline.
- Each stage of the ETL process is tested, validating the CRUD (Create, Read, Update, Delete) operations.

## Rust Files Overview

### `lib.rs`

- **Data Extraction**: The `extract_data` function downloads data from a specified URL and saves it to a local file.
  
- **Logging Queries**: The `log_query` function logs SQL queries to `queryLog.md` in markdown format.

- **Data Cleaning**: The `clean_data` function cleans and prepares data from a CSV file by converting specific columns to appropriate data types.

- **Loading Data**: The `load_data` function creates the `worker_health` table in the SQLite database and loads the cleaned data into it.

- **Data Querying**:
  - The `query_data` function retrieves and displays the top `n` records from the `worker_health` table.
  - The `query_specific_record` function queries a specific record based on `employee_id`.

- **CRUD Operations**:
  - The `create_record` function creates a new record in `worker_health` if the `Employee_ID` does not already exist.
  - The `update_record` function updates an existing record in `worker_health` if the `Employee_ID` exists.
  - The `delete_record` function deletes a record in `worker_health` based on `Employee_ID`.

### `main.rs`

- The main function orchestrates the ETL pipeline:
  - It extracts, cleans, and loads data from a CSV file.
  - It queries the top 20 records and a specific record.
  - It demonstrates creating, updating, and deleting a record in the database.

### `test.rs`

- This file contains unit tests for the CRUD operations, ensuring that records can be created, updated, and deleted successfully.

## Conclusion

This ETL pipeline is designed to efficiently handle data extraction, transformation, and loading into a SQLite database, allowing for easy querying and management of employee mental health data. 
