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

// expects nothing due to narrow date range
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
