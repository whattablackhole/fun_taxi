import { Marker, Popup } from "react-leaflet";
import L, { LatLng } from "leaflet";
import { ReactNode } from "react";

interface DriverMarkerProps {
  position: LatLng;
  children?: ReactNode;
}

export const DriverMarker = ({ position, children }: DriverMarkerProps) => {
  const carIcon = L.icon({iconUrl: "/src/assets/car-upfront.png", iconSize: [30,60]});

  return (
    <Marker position={position} icon={carIcon}>
      {children ? <Popup>{children}</Popup> : null}
    </Marker>
  );
};
