CREATE MATERIALIZED VIEW IF NOT EXISTS station_view AS
    SELECT
        stations.station_id,

        stations.name_finnish,
        stations.name_swedish,
        stations.name_english,

        stations.address_finnish,
        stations.address_swedish,

        cities.name_finnish as city_name_finnish,
        cities.name_swedish as city_name_swedish,

        stations.capacity,

        departing_journeys.amount as journeys_departing_amount,
        returning_journeys.amount as journeys_returning_amount
    FROM
        stations,
        cities,
        (SELECT departure_station_id, COUNT(*) as amount FROM journeys GROUP BY departure_station_id) as departing_journeys,
        (SELECT return_station_id, COUNT(*) as amount FROM journeys GROUP BY return_station_id) as returning_journeys
    WHERE
        stations.station_id = departing_journeys.departure_station_id
    AND
        stations.station_id = returning_journeys.return_station_id
    AND
        stations.city_id = cities.city_id
