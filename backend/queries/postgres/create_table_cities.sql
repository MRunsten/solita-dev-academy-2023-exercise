CREATE TABLE IF NOT EXISTS cities (
    city_id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name_finnish VARCHAR(255) UNIQUE NOT NULL,
    name_swedish VARCHAR(255) UNIQUE NOT NULL
);
