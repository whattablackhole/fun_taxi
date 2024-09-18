import { LatLngExpression } from "leaflet";
import { MapNode } from "../../models/MapModels";
import { Polyline } from "react-leaflet";

export const PolylineComponent = ({ nodes }: { nodes: MapNode[] | null }) => {
  if (!nodes) {
    return null;
  }
  const route = nodes.map((node) => [node.lat, node.lon] as LatLngExpression);

  return <Polyline positions={route} color="blue"></Polyline>;
};
