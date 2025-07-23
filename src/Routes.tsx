import { Routes as RoutesBase, Route } from "react-router";
import App from "./App";

function Routes() {
  return (
    <RoutesBase>
      <Route path="/" element={<App />} />
    </RoutesBase>
  );
}

export default Routes;
