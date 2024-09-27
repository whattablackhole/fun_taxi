import { useEffect, useState } from "react";
import { DriverMarker } from "../DriverMarker/DriverMarkerComponent";
import { LatLng } from "leaflet";

interface DriverPosition {
  driver_id: number;
  position: {
    lng: number;
    lat: number;
  }
}

export function NearestDrivers() {
  const [driversPositions, setDriversPositions] = useState<
    DriverPosition[] | []
  >([]);

  useEffect(() => {
    const fetchNearbyDrivers = async () => {
      const response = await fetch("http://localhost:7777/nearestdrivers", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ radius: 30, lat:52.1329457, lon: 26.1130729 }),
      });
      if (response.ok) {
        const data: DriverPosition[] = await response.json();
        setDriversPositions(data);
      }
    };

    const timer = setInterval(fetchNearbyDrivers, 3000);
    return () => clearInterval(timer);
  }, []);

  return driversPositions.map((p, i) => {
    return <DriverMarker key={i} position={new LatLng(p.position.lat, p.position.lng)}></DriverMarker>;
  });
}