#!/bin/sh

while IFS= read -r line || [ -n "$line" ]
do
  echo "INSERT INTO calibration (calibration_value) VALUES ('$line');"
done < "/docker-entrypoint-initdb.d/input.txt" > /var/lib/postgresql/data/insert_data.sql

psql -U postgres -d postgres -f /var/lib/postgresql/data/insert_data.sql
