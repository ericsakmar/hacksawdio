import { Routes as RoutesBase, Route } from "react-router";
import App from "./App";
import LoginPage from "./features/auth/LoginPage";
import HomePage from "./features/home/HomePage";
import PlayerPage from "./features/playback/PlayerPage";

function Routes() {
  return (
    <RoutesBase>
      <Route path="/" element={<App />} />
      <Route path="/login" element={<LoginPage />} />
      <Route path="/home" element={<HomePage />} />
      <Route path="/player" element={<PlayerPage />} />
    </RoutesBase>
  );
}

export default Routes;
