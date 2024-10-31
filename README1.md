# Nakiyah's Individual Project 2

![Project Logo](https://via.placeholder.com/150)  <!-- Replace with your project logo/image -->


## Overview

This project aims to analyze the impact of remote work on mental health through data extraction, transformation, and loading (ETL) processes. The application allows users to clean, load, and manage employee mental health data efficiently, facilitating insights into how remote work environments affect well-being. 

Key functionalities include querying specific records, updating employee data, creating new records, and deleting existing ones. This project is designed with Rust to leverage its performance and safety features.

## Technologies Used

- **Rust**: A systems programming language focused on safety and performance.
- **CSV**: For reading and writing comma-separated values files.
- **SQLite**: A C-language library that implements a small, fast, self-contained, high-reliability, full-featured SQL database engine.
- **Cargo**: Rust's package manager and build system.
  
## Installation

To get started with this project, you will need to have Rust and Cargo installed on your machine. Follow the instructions below:

1. **Install Rust**: If you haven’t already, install Rust by following the instructions at [rustup.rs](https://rustup.rs/).

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/nogibjj/Nakiyah_IndividualProject2.git
   cd Nakiyah_IndividualProject2
   ```

3. **Build the Project**:
   ```bash
   cargo build
   ```

## Usage

To run the project, you can use the following command:
```bash
cargo run
```

The application will execute a series of tasks to clean, load, and query the mental health data from the specified CSV file. The primary data file used is `Impact_of_Remote_Work_on_Mental_Health.csv`.

### Commands

- **Clean and Load Data**: Automatically cleans and loads data from the CSV file into the SQLite database.
- **Query Records**: Query the database for specific records.
- **Create Record**: Add new records to the database.
- **Update Record**: Modify existing records.
- **Delete Record**: Remove records from the database.

For more specific usage instructions, refer to the code comments and documentation in the source files.

## Features

- Data extraction, cleaning, and loading functionality.
- Querying of employee mental health data.
- Ability to create, update, and delete records from the database.
- Error handling and logging for better debugging.

## Tests

This project includes unit tests to ensure functionality. To run the tests, use:
```bash
cargo test
```

## Contributing

Contributions are welcome! If you would like to contribute to this project, please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/YourFeature`).
3. Make your changes and commit them (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Create a new Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contact

For any inquiries or suggestions, please contact:

- **Name**: Nakiyah Dhariwala
- **Email**: [your.email@example.com](mailto:your.email@example.com) <!-- Replace with your email -->
- **LinkedIn**: [Your LinkedIn Profile](https://www.linkedin.com/in/yourprofile) <!-- Replace with your LinkedIn URL -->
