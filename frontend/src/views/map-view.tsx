import { useEffect, useState } from "react";
import { MapContainer, TileLayer, Marker, useMap,  } from "react-leaflet";
import "leaflet/dist/leaflet.css";
import {
    LatLng,
  LatLngBounds,
  LatLngBoundsExpression,
  LatLngTuple,
} from "leaflet";

const Endpoints = () => {
    const map = useMap();
    const [startPoint, setStartPoint] = useState<LatLng | null>(null);
    const [endPoint, setEndPoint] = useState<LatLng | null>(null);
  
    useEffect(() => {
      const handleMapClick = (event: { latlng: LatLng }) => {
        if (startPoint === null) {
          setStartPoint(event.latlng);
          console.log("start: ", event.latlng)
          return;
        }
  
        if (endPoint === null) {
          setEndPoint(event.latlng);
        }
  
        console.log('end: ', event.latlng);
      };
  
      map.on('click', handleMapClick);
  
      return () => {
        map.off('click', handleMapClick);
      };
    }, [map, startPoint, endPoint]);
  
    return (
      <>
        {startPoint && <Marker position={startPoint}></Marker>}
        {endPoint && <Marker position={endPoint}></Marker>}
      </>
    );
  };
const MapComponent = () => {
  let northWest: LatLngTuple = [52.1, 26];
  let northEast: LatLngTuple = [52.15, 26.2];

  const bounds: LatLngBoundsExpression = new LatLngBounds([
    northWest,
    northEast,
  ]);

  return (
    <MapContainer
      bounds={bounds}
      maxBounds={bounds}
      zoom={13}
      style={{ height: "100vh", width: "100%" }}
    >
      <TileLayer 
        url="http://localhost:8087/tile/{z}/{x}/{y}.png"
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
      />

      <Endpoints></Endpoints>
    </MapContainer>
  );
};

export default MapComponent;
