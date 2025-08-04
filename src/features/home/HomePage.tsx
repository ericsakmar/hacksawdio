import { useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AlbumSearchResponse } from "../auth/types";
import ArrowDownIcon from "./ArrowDownIcon";
import DeleteIcon from "./DeleteIcon";
import Logo from "./Logo";
import { useDownloadStatus } from "./useDownloadStatus";
import { useFocusOnKeyPress } from "./useFocusOnKeyPress";

function HomePage() {
  const [search, setSearch] = useState("");
  const [results, setResults] = useState<AlbumSearchResponse | null>(null);
  const [isSearching, setIsSearching] = useState(false);
  const searchInputRef = useRef<HTMLInputElement>(null);
  const isDownloading = useDownloadStatus();

  useFocusOnKeyPress("/", searchInputRef);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await doSearch();
  };

  const doSearch = async () => {
    setIsSearching(true);

    const res = await invoke<AlbumSearchResponse>("search_albums", {
      search,
    });

    setIsSearching(false);
    setResults(res);
  };

  const handleDownload = async (id: string) => {
    await invoke("download_album", { albumId: id });
    await doSearch();
  };

  const handleDelete = async (id: string) => {
    await invoke("delete_album", { albumId: id });
    await doSearch();
  };

  return (
    <main className="container mx-auto p-4">
      <header>
        <Logo animated={isSearching || isDownloading} />
      </header>

      <form
        onSubmit={handleSubmit}
        className="bg-zinc-900 border-zinc-600 border-dashed border-2 p-4 my-4 flex rounded shadow-black shadow-md gap-4 focus-within:border-amber-300"
      >
        <input
          type="search"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search for an artist or album"
          required
          className="flex-grow focus:outline-none"
          ref={searchInputRef}
        />

        <button type="submit" className="focus:outline-blue-500">
          Search
        </button>
      </form>

      {results ? (
        <div>
          <p className="mb-4">{results.totalRecordCount} albums</p>

          <ul>
            {results.items.map((item) => (
              <li key={item.id}>
                <div className="grid grid-cols-[auto_1fr] gap-x-2 items-start focus-within:bg-zinc-900 rounded p-2">
                  {item.downloaded ? (
                    <button
                      className="row-span-2 mt-1 focus:outline-none focus:text-red-300"
                      onClick={() => handleDelete(item.id)}
                    >
                      <DeleteIcon />
                    </button>
                  ) : (
                    <button
                      className="row-span-2 mt-1 focus:outline-none focus:text-green-200"
                      onClick={() => handleDownload(item.id)}
                    >
                      <ArrowDownIcon />
                    </button>
                  )}
                  <div>{item.name}</div>
                  <div className="opacity-70">{item.albumArtist}</div>
                </div>
              </li>
            ))}
          </ul>
        </div>
      ) : null}
    </main>
  );
}

export default HomePage;
