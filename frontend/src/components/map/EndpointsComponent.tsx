import { LatLng } from "leaflet";
import { useEffect, useState } from "react";
import { Marker, useMap } from "react-leaflet";

export const EndpointsComponent = () => {
  const map = useMap();
  const [startPoint, setStartPoint] = useState<LatLng | null>(null);
  const [endPoint, setEndPoint] = useState<LatLng | null>(null);

  useEffect(() => {
    const handleMapClick = (event: { latlng: LatLng }) => {
      if (startPoint === null) {
        setStartPoint(event.latlng);
        return;
      }
      if (endPoint === null) {
        setEndPoint(event.latlng);
      }
    };

    map.on("click", handleMapClick);

    return () => {
      map.off("click", handleMapClick);
    };
  }, [map, startPoint, endPoint]);

  return (
    <>
      {startPoint && <Marker position={startPoint}></Marker>}
      {endPoint && <Marker position={endPoint}></Marker>}
    </>
  );
};
