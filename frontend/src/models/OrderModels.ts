import { LatLng } from "leaflet";

export interface Order {
  id: string;
  user_id: number;
  start_point: LatLng;
  end_point: LatLng;
}
