import {
  LatLng,
  LatLngBounds,
  LatLngBoundsExpression,
  LatLngTuple,
} from "leaflet";
import { useEffect, useState } from "react";
import { MapNode } from "../../models/MapModels";
import { PathFinderService } from "../../services/path-finder/path-finder";
import mockData from "../../../../response_examples/data_with_nodes.json";
import {
  CircleMarker,
  MapContainer,
  Popup,
  TileLayer,
  GeoJSON,
} from "react-leaflet";
import { v4 as uuidv4 } from "uuid";
import { EndpointsComponent } from "./EndpointsComponent";
import { PolylineComponent } from "./PolylineComponent";
import { OrdersComponent } from "./OrdersComponent";
import "./MapComponent.scss";
import { RoleBasedComponent } from "../shared/RoleBasedComponent";
import { Order } from "../../models/OrderModels";
import { NearestDrivers } from "./NearestDrivers/NearestDriversComponent";
import { GeoJsonObject } from "geojson";

interface MapComponentProps {
  handlers?: MapEventHandlers;
}

export interface MapEventHandlers {
  onOrderApply?: (order: Order) => any;
}

export const MapComponent = ({ handlers }: MapComponentProps) => {
  let northWest: LatLngTuple = [52.1, 26];
  let northEast: LatLngTuple = [52.15, 26.2];

  const navigator = new PathFinderService();
  const [nodes, setNodes] = useState<MapNode[] | null>(null);
  const [geojsonData, setGeojsonData] =
    useState<{data: GeoJsonObject, hash: string} | null>(null);
  const bounds: LatLngBoundsExpression = new LatLngBounds([
    northWest,
    northEast,
  ]);
  //temp loc
  const onEndPointSelectionChange = async (points: any) => {
    const response = await fetch("http://127.0.0.1:7777/shortest_path", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        start_lon: points.startPoint.lng,
        start_lat: points.startPoint.lat,
        end_lon: points.endPoint.lng,
        end_lat: points.endPoint.lat,
      }),
    });
    if (response) {
      const geoData = await response.json();

      if (geoData.features) {
        setGeojsonData({data: geoData, hash: uuidv4()});
      }
      console.log(geoData);
    }

    //   fetch("http://127.0.0.1:8082/api/trip_request/", {
    //     method: "POST",
    //     headers: {
    //       "Content-Type": "application/json",
    //     },
    //     body: JSON.stringify({
    //       user_id: 1,
    //       start_point: points.startPoint,
    //       end_point: points.endPoint,
    //     }),
    //   })
    //     .then((response) => {
    //       console.log(response);
    //       if (!response.ok) {
    //         throw new Error("Network response was not ok");
    //       }
    //       return response.json();
    //     })
    //     .then((data) => console.log(data))
    //     .catch((error) =>
    //       console.error("There was a problem with the fetch operation:", error)
    //     );
  };

  const onOrderApply = async (order: Order) => {
    await handlers?.onOrderApply?.(order);
  };

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
      className="map-container"
    >
      <TileLayer
        url="http://localhost:8087/tile/{z}/{x}/{y}.png"
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
      />
      <RoleBasedComponent exclude={["driver"]}>
        <EndpointsComponent
          onEndPointSelectionChange={onEndPointSelectionChange}
        ></EndpointsComponent>
      </RoleBasedComponent>

      {/* {nodes?.map((node) => (
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
      ))} */}
      <PolylineComponent nodes={nodes} />
      <RoleBasedComponent include={["driver"]}>
        <OrdersComponent onOrderApply={onOrderApply}></OrdersComponent>
      </RoleBasedComponent>
      <NearestDrivers></NearestDrivers>
      {geojsonData?.data ? (
        <GeoJSON data={geojsonData?.data} key={geojsonData?.hash}/>
      ) : null}
    </MapContainer>
  );
};
