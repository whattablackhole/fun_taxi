import L, { LatLng } from "leaflet";
import RotatedMarker from "../RotatedMarker/RotatedMarkerComponent";

interface DriverMarkerProps {
  position: LatLng;
  bearing: number;
}

export const DriverMarker = ({ position, bearing }: DriverMarkerProps) => {
  const carIcon = L.icon({
    iconUrl: "/src/assets/car-upfront.png",
    iconSize: [30, 60],
  });

  return (
    <RotatedMarker
      position={position}
      icon={carIcon}
      rotation={bearing}
    ></RotatedMarker>
  );
};
