pub(super) fn get_station_query(input: &str) -> String {
    if input.is_empty() {
        "SELECT MIN(stop_id), name \
        FROM stop \
        WHERE name LIKE '%Hbf' OR name LIKE '%Hauptbahnhof' \
        GROUP BY name;".to_string()
    } else {
        format!("SELECT MIN(stop_id), name \
        FROM stop \
        WHERE name LIKE '%{}%' \
        GROUP BY name;", input)
    }
}