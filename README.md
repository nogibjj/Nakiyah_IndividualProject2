# Nakiyah_IndividualProject2

[![Rust CI/CD Pipeline](https://github.com/nogibjj/Nakiyah_IndividualProject2/actions/workflows/rust.yml/badge.svg)](https://github.com/nogibjj/Nakiyah_IndividualProject2/actions/workflows/rust.yml)

## Youtube Video
https://youtu.be/Vh0ol19et-o

## Project Overview
```
Nakiyah_Assignment5/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── .github/
│   └── workflows/rust.yml
├── .gitignore
├── Data/
│   └── Impact_of_Remote_Work_on_Mental_Health.csv
├── mylib/
│   ├── lib.rs
│   └── main.rs
├── tests/
│   └── test.rs
├── Makefile
├── README.md
├── Cargo.lock
├── Cargo.toml
├── database1.db
└── queryLog.md
```

## Purpose of this project
This project provides functions for ETL (Extract, Transform, Load) operations and querying a SQLite database to enable efficient data analysis. 
This repository processes data from an external public dataset and stores it in a SQLite database using Rust 
(to leverage its performance and safety features). Overall, The key stages of the pipeline are as follows:

1. Extract: Data is fetched from a public GitHub repository and loaded into a local CSV file.

2. Transform & Load: The CSV file is cleaned and pre-processed to ensure the data is structured correctly and ready for analysis.
The cleaned data is then loaded into a SQLite `.db` file, where it can be efficiently queried for further analysis.

3. Querying: SQL queries are verified to ensure they return the expected results, such as retrieving the top 20 rows from a specific table.

I used the help of GitHub Copilot to convert my existing Python code into Rust. Using Co-pilot's suggestions as a base, 
I adjusted the code to improve accuracy and error handling, making the ETL pipeline more reliable for analyzing remote work’s 
impact on mental health.

## Rust Files Overview

### `lib.rs`

- **Data Extraction**: The `extract_data` function downloads data from a specified URL and saves it to a local file.

- **Data Cleaning**: The `clean_data` function cleans and prepares data from a CSV file by converting specific columns to appropriate data types.

- **Loading Data**: The `load_data` function creates the `worker_health` table in the SQLite database and loads the cleaned data into it.

- **CRUD Operations**:
    - The `create_record` function creates a new record in `worker_health` if the `Employee_ID` does not already exist.
    - The `query_data` function retrieves and displays the top `n` records from the `worker_health` table.
    - The `update_record` function updates an existing record in `worker_health` if the `Employee_ID` exists.
    - The `delete_record` function deletes a record in `worker_health` based on `Employee_ID`.

- **Logging Queries**: The `log_query` function logs SQL queries to `queryLog.md` in markdown format.

### `main.rs`

- The main function orchestrates the ETL pipeline:
  - It extracts, cleans, and loads data from a CSV file.
  - It queries the top 20 records and a specific record.
  - It demonstrates creating, updating, and deleting a record in the database.

### `test.rs`

- Each stage of the ETL process is tested, validating the CRUD (Create, Read, Update, Delete) operations.

## Preparation and Dependency Installation:
**Build the Project**:
```bash
cargo build
```

## Usage
To run the project, you can use the following command:
```bash
cargo run
```

The application will execute a series of tasks to clean, load, and query the mental health data from the specified CSV file. The primary data file used is `Impact_of_Remote_Work_on_Mental_Health.csv`.

To run the tests, use:
```bash
cargo test
```
