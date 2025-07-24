import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function LoginPage() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    console.log("Login attempt:", { username, password });
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
