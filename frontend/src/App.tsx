import "./App.css";
import { OverpassApiService } from "./services/overpass-api";
import MapView from "./views/map-view";
import 'leaflet/dist/leaflet.css';
import useWebSocket from 'react-use-websocket';
import {
  createBrowserRouter,
  RouterProvider,
  redirect,
} from "react-router-dom";
import { AuthView } from "./views/auth-view";
import { useAuthService } from "./contexts/AuthServiceContext";


function App() {
  const authService = useAuthService();

  const checkAuthentication = async () => {
    const isAuthenticated = await authService.authenticate();
    if (!isAuthenticated) {
      return redirect("/auth");
    }
    return null;
  };

  const overpassApiService = new OverpassApiService();

  
  useWebSocket("ws://localhost:8000/ws/default/", {
    share: true,
    onOpen: () => console.log('opened'),
    shouldReconnect: () => true,
  });

  // const connectDriverSocket = ()=>{
  //   const driverSocketService = new WebSocketService("ws://localhost:8000")
  // }
  
  const router = createBrowserRouter([
    {
      path: "/",
      loader: checkAuthentication,
      element: (
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
      ),
    },
    {
      path: "auth",
      element: <AuthView></AuthView>,
    },
  ]);

  return <RouterProvider router={router}></RouterProvider>

}
export default App;
