CREATE TABLE IF NOT EXISTS stations (
    station_id INT PRIMARY KEY UNIQUE NOT NULL,

    city_id INT NOT NULL,
    operator_id INT NOT NULL,

    name_finnish VARCHAR(255) NOT NULL,
    name_swedish VARCHAR(255) NOT NULL,
    name_english VARCHAR(255) NOT NULL,

    address_finnish VARCHAR(255) NOT NULL,
    address_swedish VARCHAR(255) NOT NULL,

    capacity INT NOT NULL,
    latitude_north DOUBLE PRECISION NOT NULL,
    longitude_east DOUBLE PRECISION NOT NULL,

    CONSTRAINT fk_city_id FOREIGN KEY(city_id) REFERENCES cities(city_id),
    CONSTRAINT fk_operator_id FOREIGN KEY(operator_id) REFERENCES station_operators(operator_id)
);
