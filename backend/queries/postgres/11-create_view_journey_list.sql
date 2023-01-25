CREATE VIEW journey_list_view AS
    SELECT
        journeys.id,
        journeys.departure_date,
        journeys.return_date,

        journeys.distance_meters / 1000 as distance_kilometers,
        journeys.duration_seconds / 60.0 as duration_minutes,

        departure_stations.station_id as departure_station_id,
        departure_stations.name_finnish as departure_station_name_finnish,
        departure_stations.name_swedish as departure_station_name_swedish,
        departure_stations.name_english as departure_station_name_english,

        return_stations.station_id as return_station_id,
        return_stations.name_finnish as return_station_name_finnish,
        return_stations.name_swedish as return_station_name_swedish,
        return_stations.name_english as return_station_name_english
    FROM
        journeys,
        stations as departure_stations,
        stations as return_stations
    WHERE
        journeys.departure_station_id = departure_stations.station_id
    AND
        journeys.return_station_id = return_stations.station_id
