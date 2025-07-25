import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router";

function LoginPage() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    try {
      await invoke("authenticate_user_by_name_cmd", {
        username,
        password,
      });
      navigate("/home");
    } catch (error) {
      console.error("Login failed:", error);
      // Handle login failure (e.g., show an error message)
      return;
    }
  };

  return (
    <main className="container">
      <h1 className="text-xl">hacksawdio login</h1>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          placeholder="Username"
          required
        />

        <input
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          placeholder="Password"
          required
        />

        <button type="submit">Login</button>
      </form>
    </main>
  );
}

export default LoginPage;
