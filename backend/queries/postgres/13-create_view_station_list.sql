CREATE MATERIALIZED VIEW IF NOT EXISTS station_list_view AS
    SELECT
        stations.station_id,

        stations.name_finnish,
        stations.name_swedish,
        stations.name_english,

        stations.capacity,

        cities.name_finnish as city_name_finnish,
        cities.name_swedish as city_name_swedish,

        station_operators.operator_name
    FROM
        stations,
        cities,
        station_operators
    WHERE
        stations.city_id = cities.city_id
    AND
        stations.operator_id = station_operators.operator_id
