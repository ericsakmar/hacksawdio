import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router";
import Routes from "./Routes";
import "./App.css";
import { PlaybackProvider } from "./features/playback/PlaybackProvider";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <PlaybackProvider>
      <BrowserRouter>
        <Routes />
      </BrowserRouter>
    </PlaybackProvider>
  </React.StrictMode>
);
