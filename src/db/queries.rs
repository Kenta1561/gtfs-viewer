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

pub(super) const SERVICE_QUERY: &str = "SELECT s.*, se.service_date, se.exception_type \
    FROM service s \
    LEFT JOIN service_exception se \
    ON se.service_id = s.service_id;";

pub(super) const STOP_QUERY: &str = "SELECT \
    st.arrival_time, st.departure_time, t.trip_id, s.service_id, t.short_name, t.headsign \
    FROM stop_time st \
    INNER JOIN trip t ON t.trip_id = st.trip_id \
    INNER JOIN service s ON s.service_id = t.service_id \
    INNER JOIN route r ON r.route_id = t.route_id \
    INNER JOIN agency a ON a.agency_id = r.agency_id
    WHERE st.stop_id = ?1;";
