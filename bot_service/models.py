from collections import namedtuple
from dataclasses import dataclass
from typing import List, Tuple

Coordinates = Tuple[float, float]

@dataclass
class Geometry:
    type: str
    coordinates: List[Coordinates]

@dataclass
class Properties:
    osm_id: int

@dataclass
class Feature:
    type: str
    geometry: Geometry
    properties: Properties

@dataclass
class FeatureCollection:
    type: str
    features: List[Feature] | None


Position = namedtuple('Position', ['lat', 'lng'])

@dataclass
class DriverGeoPosition:
    driver_id: int
    position: Position
