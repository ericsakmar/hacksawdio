import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router";
import Routes from "./Routes";
import "./App.css";
import { PlaybackProvider } from "./features/playback/PlaybackProvider";
import { OnlineStatusProvider } from "./features/OnlineStatusProvider";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <OnlineStatusProvider>
      <PlaybackProvider>
        <BrowserRouter>
          <Routes />
        </BrowserRouter>
      </PlaybackProvider>
    </OnlineStatusProvider>
  </React.StrictMode>
);
