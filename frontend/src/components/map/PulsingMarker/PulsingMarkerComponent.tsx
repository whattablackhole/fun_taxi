import { Marker, Popup } from "react-leaflet";
import L, { LatLng } from "leaflet";
import "./PulsingMarkerComponent.css";
import { ReactNode } from "react";


interface PulsingMarkerProps {
  position: LatLng;
  children?: ReactNode; // Define children as an optional prop
}

export const PulsingMarker = ({ position, children }: PulsingMarkerProps) => {
  const pulsingIcon = L.divIcon({
    html: '<div class="pulsing-icon"></div>',
    className: "pulsing-icon-container",
  });

  return <Marker position={position} icon={pulsingIcon}>
    <Popup>
      {children}
    </Popup>
  </Marker>;
};
