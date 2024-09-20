import { useState, useEffect } from "react";
import useWebSocket from "react-use-websocket";
import { LatLng } from "leaflet";
import { PulsingMarker } from "./PulsingMarker/PulsingMarkerComponent";
import {
  haversineDistance,
  truncateToDecimalPlaces,
} from "../../utils/distance";

interface Order {
  user_id: number;
  start_point: LatLng;
  end_point: LatLng;
}

export const OrdersComponent = () => {
  const [orders, setOrders] = useState<Order[]>([]);
  const { lastMessage } = useWebSocket("ws://localhost:8000/ws/default/");
  useEffect(() => {
    if (lastMessage) {
      const data: any = JSON.parse(lastMessage.data);
      if (data.type === "transportation_request") {
        const newOrder: Order = JSON.parse(data.message);
        setOrders((prevOrders) => [...prevOrders, newOrder]);
      }
    }
  }, [lastMessage]);
  return orders.map((order, index) => {
    let distance = truncateToDecimalPlaces(
      haversineDistance(
        order.start_point.lat,
        order.start_point.lng,
        order.end_point.lat,
        order.end_point.lng
      ),
      2
    );
    return (
      <PulsingMarker key={index} position={order.start_point}>
        <div>
          <div>User id: {order.user_id}</div>
          <div>Min trip distance: {distance} km</div>
          <div>
            <button>Take Order</button>
            <button>Deny Order</button>
          </div>
        </div>
      </PulsingMarker>
    );
  });
};
