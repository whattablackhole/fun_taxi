import "./App.css";
import { OverpassApiService } from "./services/overpass-api";
import "leaflet/dist/leaflet.css";
import useWebSocket from "react-use-websocket";
import {
  createBrowserRouter,
  RouterProvider,
  redirect,
} from "react-router-dom";
import { AuthView } from "./views/auth-view";
import { useAuthService } from "./contexts/AuthServiceContext";
import IndexView from "./views/index-view";
import { useEffect, useState } from "react";

function App() {
  const { authService, userData } = useAuthService();
  const [socketUrl, setSocketUrl] = useState<string | null>(null);

  useWebSocket(socketUrl, {
    share: true,
    onOpen: () => console.log("opened"),
    shouldReconnect: () => true,
  });

  const checkAuthentication = async () => {
    const isAuthenticated = await authService.authenticate();
    if (!isAuthenticated) {
      return redirect("/auth");
    }
    return null;
  };

  const overpassApiService = new OverpassApiService();

  useEffect(() => {
    const result = authService.checkUserRole("driver");
    if (result) {
      setSocketUrl("ws://localhost:8000/ws/default/");
    } else {
      setSocketUrl("ws://localhost:8081/ws/default/");
    }
  }, [userData?.roles]);

  const router = createBrowserRouter([
    {
      path: "/",
      loader: checkAuthentication,
      element: (
        <>
          <IndexView />
          <div className="card">
            <button
              onClick={async () => overpassApiService.makeSimpleRequest()}
            >
              Make Request To OverPassAPI
            </button>
          </div>
        </>
      ),
    },
    {
      path: "/auth",
      element: <AuthView></AuthView>,
    },
  ]);

  return <RouterProvider router={router}></RouterProvider>;
}
export default App;
