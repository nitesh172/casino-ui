use chrono::prelude::*;

pub fn format_date_to_readable(input_date: String, format: &str) -> String {
    let parsed_datetime = input_date.parse::<DateTime<Utc>>().expect("invalid date");

    let formatted_str = parsed_datetime.format(format).to_string();
    formatted_str
}

pub fn format_date_for_backend(input_date: String) -> String {
    let formatted_date = format!("{}:00Z", input_date);
    formatted_date
}