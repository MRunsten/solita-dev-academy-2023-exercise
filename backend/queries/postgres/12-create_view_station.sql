CREATE MATERIALIZED VIEW IF NOT EXISTS station_view AS
    SELECT
        stations.station_id,

        stations.name_finnish,
        stations.name_swedish,
        stations.name_english,

        stations.address_finnish,
        stations.address_swedish,

        cities.name_finnish AS city_name_finnish,
        cities.name_swedish AS city_name_swedish,

        station_operators.operator_name AS operator_name,

        stations.capacity,

        CASE
            WHEN departing_journeys.amount IS NULL THEN 0
            ELSE departing_journeys.amount
            END AS journeys_departing_amount,

        CASE
            WHEN returning_journeys.amount IS NULL THEN 0
            ELSE returning_journeys.amount
            END AS journeys_returning_amount

    FROM
        stations
            LEFT JOIN (SELECT departure_station_id, COUNT(*) as amount FROM journeys GROUP BY departure_station_id) AS departing_journeys
                      ON stations.station_id = departing_journeys.departure_station_id

            LEFT JOIN (SELECT return_station_id, COUNT(*) as amount FROM journeys GROUP BY return_station_id) AS returning_journeys
                      ON stations.station_id = returning_journeys.return_station_id,
        cities,
        station_operators
    WHERE
            stations.city_id = cities.city_id
      AND
            stations.operator_id = station_operators.operator_id
