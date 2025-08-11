import { Routes as RoutesBase, Route } from "react-router";
import App from "./App";
import LoginPage from "./features/auth/LoginPage";
import SearchPage from "./features/search/SearchPage";
import PlayerPage from "./features/playback/PlayerPage";
import MainLayout from "./features/MainLayout";

function Routes() {
  return (
    <RoutesBase>
      <Route path="/" element={<App />} />
      <Route path="/login" element={<LoginPage />} />

      <Route element={<MainLayout />}>
        <Route path="/search" element={<SearchPage />} />
        <Route path="/player" element={<PlayerPage />} />
      </Route>
    </RoutesBase>
  );
}

export default Routes;
