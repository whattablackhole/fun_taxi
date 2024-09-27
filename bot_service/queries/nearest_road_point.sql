SELECT 
       ST_AsText(ST_Transform(ST_ClosestPoint(l.way, ST_Transform(ST_SetSRID(ST_MakePoint(26.1130729, 52.1329457), 4326), 3857)), 4326)) AS closest_point,
       ST_Distance(l.way, ST_Transform(ST_SetSRID(ST_MakePoint(26.1130729, 52.1329457), 4326), 3857)) AS distance
FROM planet_osm_line l
WHERE l.highway IN ('motorway', 'trunk', 'primary', 'secondary', 'residential')
ORDER BY distance
LIMIT 1;