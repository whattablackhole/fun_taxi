import { useEffect } from "react";
import "./App.css";
import { OverpassApiService } from "./services/overpass-api";
import { WebSocketService } from "./services/websocket-connection";
import MapView from "./views/map-view";
import 'leaflet/dist/leaflet.css';
import useWebSocket from 'react-use-websocket';


function App() {
  const overpassApiService = new OverpassApiService();

  useWebSocket("ws://localhost:8000/ws/default/", {
    share: true,
    onOpen: () => console.log('opened'),
    shouldReconnect: () => true,
  });

  // const connectDriverSocket = ()=>{
  //   const driverSocketService = new WebSocketService("ws://localhost:8000")
  // }

  return (
    <>
      <MapView/>
      <div className="card">
        <button onClick={() => overpassApiService.makeSimpleRequest()}>
          Make Request To OverPassAPI
        </button>
        {/* <button onClick={() => connectDriverSocket()}>
          Connect As Driver
        </button> */}
      </div>
    </>
  );
}

export default App;
