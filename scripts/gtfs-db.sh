#!/bin/bash
# Usage: ./gtfs-db.sh <zip_path>
# Make sure the script files are in the same directory as the ZIP file.

# Archive extraction
echo "Extracting source files from ZIP archive..."
7z x $1 -ogtfs
echo "Extraction done."

cd gtfs

# Delete unused files
echo "Removing unused files."
rm frequencies.txt shapes.txt transfers.txt *.pdf

# Create & import
echo "Reading schema.sql..."
sqlite3 data.db ".read ../schema.sql"
echo "Done."

echo "Reading import.sql..."
sqlite3 data.db ".read ../import.sql"
echo "Done."

# Cleanup
echo "Reading cleanup.sql..."
sqlite3 data.db ".read ../cleanup.sql"
echo "Done."

# Vacuum
echo "Starting vacuum..."
sqlite3 data.db "VACUUM;"
echo "Done."

