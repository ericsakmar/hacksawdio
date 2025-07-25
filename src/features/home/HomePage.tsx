import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AlbumSearchResponse } from "../auth/types";

function HomePage() {
  const [search, setSearch] = useState("");
  const [results, setResults] = useState<AlbumSearchResponse | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const res = await invoke<AlbumSearchResponse>("search_albums", {
      search,
    });

    console.log("Search results:", res);

    setResults(res);
  };

  return (
    <main className="container">
      <h1 className="text-xl">hacksawdio - search</h1>
      <form onSubmit={handleSubmit}>
        <input
          type="search"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search for an artist or album"
          required
        />

        <button type="submit">Search</button>
      </form>

      {results ? (
        <div>
          <h2 className="text-lg">Results</h2>
          <ul>
            {results.items.map((item) => (
              <li key={item.id}>
                {item.name} {item.albumArtist ? `by ${item.albumArtist}` : ""}
                <button disabled={item.downloaded}>download</button>
              </li>
            ))}
          </ul>
          <p>Total results: {results.totalRecordCount}</p>
        </div>
      ) : null}
    </main>
  );
}

export default HomePage;
