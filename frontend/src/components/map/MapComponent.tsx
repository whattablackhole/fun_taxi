import { LatLngBounds, LatLngBoundsExpression, LatLngTuple } from "leaflet";
import { useEffect, useState } from "react";
import { MapNode } from "../../models/MapModels";
import { PathFinderService } from "../../services/path-finder/path-finder";
import mockData from "../../../../response_examples/data_with_nodes.json";
import { CircleMarker, MapContainer, Popup, TileLayer } from "react-leaflet";
import { EndpointsComponent } from "./EndpointsComponent";
import { PolylineComponent } from "./PolylineComponent";

export const MapComponent = () => {
  let northWest: LatLngTuple = [52.1, 26];
  let northEast: LatLngTuple = [52.15, 26.2];
  const navigator = new PathFinderService();

  const [nodes, setNodes] = useState<MapNode[] | null>(null);

  const bounds: LatLngBoundsExpression = new LatLngBounds([
    northWest,
    northEast,
  ]);

  useEffect(() => {
    navigator.buildGraph(mockData.elements as any);

    let path = navigator.findShortestPath(
      { lat: 52.12467561521258, lon: 26.113214492797855 },
      { lat: 52.13841380914448, lon: 26.118493080139164 }
    );
    
    setNodes(path);

    console.log(path);
  }, []);

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

      <EndpointsComponent></EndpointsComponent>

      {nodes?.map((node) => (
        <CircleMarker
          key={node.id}
          center={[node.lat, node.lon]}
          radius={5}
          color="blue"
          fillColor="#03f"
          fillOpacity={0.7}
        >
          <Popup>Node ID: {node.id}</Popup>
        </CircleMarker>
      ))}
      <PolylineComponent nodes={nodes} />
    </MapContainer>
  );
};
