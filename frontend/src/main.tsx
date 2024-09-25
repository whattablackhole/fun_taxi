import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import "leaflet/dist/leaflet.css";
import { AuthServiceProvider } from "./contexts/AuthServiceContext.tsx";

createRoot(document.getElementById("root")!).render(
  // TODO: Add back StrictMode later
  <StrictMode>
    <AuthServiceProvider>
      <App />
    </AuthServiceProvider>
  </StrictMode>
);
