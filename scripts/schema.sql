-- SQL file to create GTFS database schema

PRAGMA foreign_keys = ON;

CREATE TABLE service (
    service_id INT PRIMARY KEY,
    monday INT NOT NULL,
    tuesday INT NOT NULL,
    wednesday INT NOT NULL,
    thursday INT NOT NULL,
    friday INT NOT NULL,
    saturday INT NOT NULL,
    sunday INT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL
);

CREATE TABLE service_exception (
    service_id INT,
    service_date TEXT,
    exception_type INT NOT NULL,
    PRIMARY KEY (service_id, service_date),
    FOREIGN KEY (service_id) REFERENCES service (service_id)
);

CREATE TABLE agency (
    agency_id INT PRIMARY KEY,
    name TEXT NOT NULL,
    url TEXT,       -- tmp
    timezone TEXT,  -- tmp
    lang TEXT,      -- tmp
    phone TEXT      -- tmp
);

CREATE TABLE route (
    route_id TEXT PRIMARY KEY NOT NULL,
    agency_id INT,
    short_name TEXT,
    long_name TEXT,
    route_type INT NOT NULL,
    route_color TEXT,       -- tmp
    route_text_color TEXT,  -- tmp
    route_desc TEXT,        -- tmp
    FOREIGN KEY (agency_id) REFERENCES agency (agency_id)
);

CREATE TABLE stop (
    stop_id TEXT PRIMARY KEY NOT NULL,
    code TEXT,                  -- tmp
    name TEXT,
    description TEXT,           -- tmp
    latitude REAL,
    longitude REAL,
    location_type INT,          -- tmp
    parent_station TEXT,        -- tmp
    wheelchair_boarding INT,    -- tmp
    platform_code TEXT          -- tmp
);

CREATE TABLE trip (
    route_id TEXT,
    service_id TEXT,
    trip_id INT PRIMARY KEY,
    headsign TEXT,
    short_name TEXT,
    direction_id INT,
    block_id INT,               -- tmp
    shape_id INT,               -- tmp
    wheelchair_accessible INT,  -- tmp
    bikes_allowed INT,          -- tmp
    FOREIGN KEY (route_id) REFERENCES route (route_id),
    FOREIGN KEY (service_id) REFERENCES service (service_id)
);

CREATE TABLE stop_time (
    trip_id INT,
    arrival_time TEXT,
    departure_time TEXT,
    stop_id TEXT,
    stop_sequence INT,
    pickup_type INT,    -- tmp
    drop_off_type INT,  -- tmp
    stop_headsign INT,  -- tmp
    PRIMARY KEY (trip_id, stop_sequence),
    FOREIGN KEY (trip_id) REFERENCES trip (trip_id),
    FOREIGN KEY (stop_id) REFERENCES stop (stop_id)
);

