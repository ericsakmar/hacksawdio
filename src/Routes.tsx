import { Routes as RoutesBase, Route } from "react-router";
import App from "./App";
import LoginPage from "./features/auth/LoginPage";
import HomePage from "./features/home/HomePage";

function Routes() {
  return (
    <RoutesBase>
      <Route path="/" element={<App />} />
      <Route path="/login" element={<LoginPage />} />
      <Route path="/home" element={<HomePage />} />
    </RoutesBase>
  );
}

export default Routes;
