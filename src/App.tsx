import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router";
import { SessionResponse } from "./features/auth/types";

function App() {
  const navigate = useNavigate();

  const checkAuth = async () => {
    const session = await invoke<SessionResponse>("get_session");
    if (session.authenticated) {
      // TODO
      // navigate("/dashboard");
    } else {
      navigate("/login");
    }
  };

  useEffect(() => {
    checkAuth();
  }, [checkAuth]);

  return (
    <main className="container">
      <h1 className="text-xl">HACKSAWDIO</h1>
    </main>
  );
}

export default App;
