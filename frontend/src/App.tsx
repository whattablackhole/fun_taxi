import { useEffect } from "react";
import "./App.css";
import { OverpassApiService } from "./services/overpass-api";
import { WebSocketService } from "./services/websocket-connection";
import MapView from "./views/map-view";
import 'leaflet/dist/leaflet.css';


function App() {
  const overpassApiService = new OverpassApiService();

  useEffect(() => {
    const socketService = new WebSocketService();

    return () => {
      socketService.disconnect();
    };
  });

  const connectDriverSocket = ()=>{
    const driverSocketService = new WebSocketService("ws://localhost:8000")
  }

  return (
    <>
      <MapView/>
      <div className="card">
        <button onClick={() => overpassApiService.makeSimpleRequest()}>
          Make Request To OverPassAPI
        </button>
        <button onClick={() => connectDriverSocket()}>
          Connect As Driver
        </button>
      </div>
    </>
  );
}

export default App;
