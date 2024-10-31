// src/tests.rs
use project2::{create_record, delete_record, query_data, query_specific_record, update_record};

#[test]
fn test_create_record() {
    // Set up test data for creating a new record
    let employee_id = 201;
    let age = 30;
    let job_role = "Tester";
    let industry = "Tech";
    let years_of_experience = 5;
    let work_location = "Remote";
    let hours_worked_per_week = 40;
    let mental_health_condition = "None";
    let access_to_resources = true;

    // Create a new record
    let create_result = create_record(
        employee_id,
        age,
        job_role,
        industry,
        years_of_experience,
        work_location,
        hours_worked_per_week,
        mental_health_condition,
        access_to_resources,
    );

    assert!(create_result.is_ok());

    // Verify the record was created
    let query_result = query_specific_record(employee_id);
    assert!(query_result.is_ok());
}

#[test]
fn test_update_record() {
    // Set up test data for updating an existing record
    let employee_id = 201;
    let new_age = 32;
    let new_job_role = "Senior Tester";
    let new_industry = "Tech";
    let new_years_of_experience = 6;
    let new_work_location = "On-site";
    let new_hours_worked_per_week = 42;
    let new_mental_health_condition = "None";
    let new_access_to_resources = false;

    // Update the existing record
    let update_result = update_record(
        employee_id,
        new_age,
        new_job_role,
        new_industry,
        new_years_of_experience,
        new_work_location,
        new_hours_worked_per_week,
        new_mental_health_condition,
        new_access_to_resources,
    );

    assert!(update_result.is_ok());

    // Verify the update
    let query_result = query_specific_record(employee_id);
    assert!(query_result.is_ok());
}

#[test]
fn test_delete_record() {
    let employee_id = 201;

    // Delete the record
    let delete_result = delete_record(employee_id);
    assert!(delete_result.is_ok(), "Failed to delete record");
}

#[test]
fn test_general_query() {
    // Set up a query to retrieve data
    let num_records = 20;
    let query_result = query_data(num_records);

    assert!(query_result.is_ok());
}
