PRAGMA foreign_keys = ON;

-- agency
ALTER TABLE agency DROP COLUMN url;
ALTER TABLE agency DROP COLUMN timezone;
ALTER TABLE agency DROP COLUMN lang;
ALTER TABLE agency DROP COLUMN phone;

-- route
ALTER TABLE route DROP COLUMN route_color;
ALTER TABLE route DROP COLUMN route_text_color;
ALTER TABLE route DROP COLUMN route_desc;

-- stop
ALTER TABLE stop DROP COLUMN code;
ALTER TABLE stop DROP COLUMN description;
ALTER TABLE stop DROP COLUMN location_type;
ALTER TABLE stop DROP COLUMN parent_station;
ALTER TABLE stop DROP COLUMN wheelchair_boarding;
ALTER TABLE stop DROP COLUMN platform_code;

-- trip
ALTER TABLE trip DROP COLUMN block_id;
ALTER TABLE trip DROP COLUMN shape_id;
ALTER TABLE trip DROP COLUMN wheelchair_accessible;
ALTER TABLE trip DROP COLUMN bikes_allowed;

-- stop_time
ALTER TABLE stop_time DROP COLUMN pickup_type;
ALTER TABLE stop_time DROP COLUMN drop_off_type;
ALTER TABLE stop_time DROP COLUMN stop_headsign;

