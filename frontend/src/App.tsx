import { useEffect } from "react";
import "./App.css";
import { OverpassApiService } from "./services/overpass-api";
import { WebSocketService } from "./services/websocket-connection";

function App() {
  const overpassApiService = new OverpassApiService();

  useEffect(() => {
    const socketService = new WebSocketService();

    return () => {
      socketService.disconnect();
    };
  });
  return (
    <>
      <div className="card">
        <button onClick={() => overpassApiService.makeSimpleRequest()}>
          Make Request To OverPassAPI
        </button>
      </div>
    </>
  );
}

export default App;
