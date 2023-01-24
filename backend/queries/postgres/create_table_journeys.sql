CREATE TABLE IF NOT EXISTS journeys (
    id SERIAL PRIMARY KEY,

    departure_date TIMESTAMP NOT NULL,
    departure_station_id INT NOT NULL,

    return_date TIMESTAMP NOT NULL,
    return_station_id INT NOT NULL,

    distance_meters NUMERIC NOT NULL,
    duration_seconds NUMERIC NOT NULL,

    CONSTRAINT fk_departure_station_id FOREIGN KEY(departure_station_id) REFERENCES stations(station_id),
    CONSTRAINT fk_return_station_id FOREIGN KEY(return_station_id) REFERENCES stations(station_id)
);
