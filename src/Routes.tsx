import { Routes as RoutesBase, Route } from "react-router";
import App from "./App";
import LoginPage from "./features/auth/LoginPage";

function Routes() {
  return (
    <RoutesBase>
      <Route path="/" element={<App />} />
      <Route path="/login" element={<LoginPage />} />
    </RoutesBase>
  );
}

export default Routes;
