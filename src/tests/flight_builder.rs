use crate::lib;

#[test]
pub fn nominal_case() {
    let expected: Vec<(String, String)> = vec![
        ("2024-04-15".to_string(), "2024-04-21".to_string()),
        ("2024-04-22".to_string(), "2024-04-28".to_string()),
        ("2024-04-29".to_string(), "2024-05-05".to_string()),
        ("2024-05-06".to_string(), "2024-05-12".to_string()),
    ];
    assert_eq!(lib:: get_weekday_combinations(
        "2024-04-15",
        "2024-05-15",
        "monday",
        "sunday",
    ), expected);
}

// Return no combinations in case the end date never procedes the start date within the narrow time range
#[test]
pub fn narrow_date_range_case() {
    let expected: Vec<(String, String)> = vec![];
    assert_eq!(lib:: get_weekday_combinations(
        "2024-04-15",
        "2024-04-20",
        "monday",
        "sunday",
    ), expected);
}

// This test case addresses a specific bug where, on a Saturday, the calculated end date
// would be the following Sunday, which incorrectly precedes the start date if the start
// date is the following Monday.
// The test ensures that the first calculated end date is always after the start date.
#[test]
pub fn end_date_always_after_start_state_case() {

    let expected: Vec<(String, String)> = vec![
        ("2024-04-15".to_string(), "2024-04-21".to_string()),
        ("2024-04-22".to_string(), "2024-04-28".to_string()),
        ("2024-04-29".to_string(), "2024-05-05".to_string()),
        ("2024-05-06".to_string(), "2024-05-12".to_string()),
    ];
    assert_eq!(lib:: get_weekday_combinations(
        "2024-04-13",
        "2024-05-13",
        "monday",
        "sunday",
    ), expected);
}