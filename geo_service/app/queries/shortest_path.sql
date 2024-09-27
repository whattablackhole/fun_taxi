WITH start_point AS (
    SELECT ST_ClosestPoint(way, ST_Transform(ST_SetSRID(ST_MakePoint(lon, lat), 4326), 3857)) AS snapped_point,
           way, source, target
    FROM planet_osm_roads
    WHERE osm_id > 0
    ORDER BY ST_Distance(way, ST_Transform(ST_SetSRID(ST_MakePoint(lon, lat), 4326), 3857))
    LIMIT 1
),
end_point AS (
    SELECT ST_ClosestPoint(way, ST_Transform(ST_SetSRID(ST_MakePoint(lon, lat), 4326), 3857)) AS snapped_point,
           way, source, target
    FROM planet_osm_roads
    WHERE osm_id > 0
    ORDER BY ST_Distance(way, ST_Transform(ST_SetSRID(ST_MakePoint(lon, lat), 4326), 3857))
    LIMIT 1
),
start_node AS (
    SELECT CASE
        WHEN ST_Distance(snapped_point, ST_StartPoint(way)) < ST_Distance(snapped_point, ST_EndPoint(way))
        THEN source ELSE target
    END AS id
    FROM start_point
),
end_node AS (
    SELECT CASE
        WHEN ST_Distance(snapped_point, ST_StartPoint(way)) < ST_Distance(snapped_point, ST_EndPoint(way))
        THEN source ELSE target
    END AS id
    FROM end_point
),
path AS (
    SELECT * FROM pgr_dijkstra(
        'SELECT osm_id AS id, source, target, ST_Length(way) AS cost FROM planet_osm_roads',
        (SELECT id FROM start_node),
        (SELECT id FROM end_node),
        directed := false
    )
),
features AS (
    SELECT 
        r.osm_id,
        ST_AsGeoJSON(ST_Transform(r.way, 4326))::jsonb AS geometry
    FROM planet_osm_roads r
    JOIN path p ON r.osm_id = p.edge
    ORDER BY p.seq
)

SELECT jsonb_build_object(
    'type', 'FeatureCollection',
    'features', jsonb_agg(
      jsonb_build_object(
        'type', 'Feature',
        'geometry', geometry,
        'properties', jsonb_build_object('osm_id', osm_id)
      )
    )
) AS geojson
FROM features;