import { LatLng } from "leaflet";
import { useEffect, useState, useCallback } from "react";
import { Marker, useMap } from "react-leaflet";

export const EndpointsComponent = ({ onEndPointSelectionChange }: { onEndPointSelectionChange: (points: { startPoint: LatLng | null, endPoint: LatLng | null }) => void }) => {
  const map = useMap();
  const [startPoint, setStartPoint] = useState<LatLng | null>(null);
  const [endPoint, setEndPoint] = useState<LatLng | null>(null);

  const handleMapClick = useCallback((event: { latlng: LatLng }) => {
    if (startPoint && endPoint) {
      setStartPoint(event.latlng);
      setEndPoint(null);
    } else if (!startPoint) {
      setStartPoint(event.latlng);
    } else {
      setEndPoint(event.latlng);
      onEndPointSelectionChange({ startPoint, endPoint: event.latlng });
    }
  }, [startPoint, endPoint, onEndPointSelectionChange]);

  useEffect(() => {
    map.on("click", handleMapClick);
    return () => {
      map.off("click", handleMapClick);
    };
  }, [map, handleMapClick]);

  const resetPoints = () => {
    setStartPoint(null);
    setEndPoint(null);
  };

  return (
    <>
      {startPoint && <Marker position={startPoint}></Marker>}
      {endPoint && <Marker position={endPoint}></Marker>}
      <button onClick={resetPoints}>Reset Points</button>
    </>
  );
};