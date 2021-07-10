.mode csv

PRAGMA foreign_keys = ON;

.import --skip 1 agency.txt agency
.import --skip 1 routes.txt route
.import --skip 1 stops.txt stop
.import --skip 1 calendar.txt service
.import --skip 1 calendar_dates.txt service_exception
.import --skip 1 trips.txt trip
.import --skip 1 stop_times.txt stop_time

