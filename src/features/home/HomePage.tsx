import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Logo from "./Logo";
import { useDownloadStatus } from "./useDownloadStatus";
import DownloadIcon from "../components/DownloadIcon";
import CircleCheckIcon from "../components/CircleCheckIcon";
import { useSearch } from "./useSearch";
import { useHotkeys } from "react-hotkeys-hook";
import OnlineIcon from "../components/OnlineIcon";
import OfflineIcon from "../components/OfflineIcon";

function HomePage() {
  const searchInputRef = useRef<HTMLInputElement>(null);
  const resultsRef = useRef<HTMLUListElement>(null);
  const isDownloading = useDownloadStatus();
  const [focusedAlbumId, setFocusedAlbumId] = useState<string | null>(null);
  const [isOnline, setIsOnline] = useState(false);

  const {
    executeSearch,
    isSearching,
    limit,
    offset,
    results,
    search,
    setDownloaded,
    setSearch,
  } = useSearch(isOnline);

  // focus the search input when "slash" is pressed
  useHotkeys(
    "slash",
    () => {
      searchInputRef.current?.focus();
      searchInputRef.current?.select();
    },
    { preventDefault: true }
  );

  // blur the search input when "Escape" is pressed
  useHotkeys(
    "esc",
    () => {
      if (document.activeElement === searchInputRef.current) {
        searchInputRef.current?.blur();
      }
    },
    {
      enableOnFormTags: ["INPUT", "TEXTAREA"],
    }
  );

  // toggle online/offline mode with "cmd + o"
  useHotkeys(
    "meta+o",
    () => {
      setIsOnline((prev) => !prev);
    },
    {
      preventDefault: true,
      enableOnFormTags: ["INPUT", "TEXTAREA"],
    }
  );

  // keeps focus between searches
  useEffect(() => {
    if (results && results.items.length > 0) {
      setFocusedAlbumId(results.items[0].id);
    } else {
      setFocusedAlbumId(null);
    }
  }, [results]);

  // sets the focus
  useEffect(() => {
    if (focusedAlbumId && resultsRef.current) {
      const liElement = resultsRef.current.querySelector(
        `li[data-album-id='${focusedAlbumId}']`
      );

      const buttonToFocus = liElement?.querySelector("button");
      buttonToFocus?.focus();
    }
  }, [focusedAlbumId]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    executeSearch(0);
  };

  const handleDownload = async (id: string) => {
    await invoke("download_album", { albumId: id });
    setDownloaded(id, true);
    setFocusedAlbumId(id);
  };

  const handleDelete = async (id: string) => {
    await invoke("delete_album", { albumId: id });
    setDownloaded(id, false);
    setFocusedAlbumId(id);
  };

  const handleOnlineToggle = () => {
    setIsOnline((prev) => !prev);
  };

  return (
    <main className="container mx-auto p-4">
      <header className="relative">
        <Logo animated={isSearching || isDownloading} />
        <button
          onClick={handleOnlineToggle}
          className="absolute top-2 right-0 opacity-70 focus:opacity-100 hover:opacity-100"
        >
          {isOnline ? <OnlineIcon /> : <OfflineIcon />}
        </button>
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
          <p className="mb-4 ml-2 opacity-70">
            {results.totalRecordCount < limit
              ? `${results.totalRecordCount} albums`
              : `${offset + 1} to ${Math.min(
                  offset + limit,
                  results.totalRecordCount
                )} of ${results.totalRecordCount} albums`}
          </p>

          <ul ref={resultsRef}>
            {results.items.map((item) => (
              <li key={item.id} data-album-id={item.id}>
                <div className="grid grid-cols-[auto_1fr] gap-x-2 items-start focus-within:bg-zinc-900 rounded p-2">
                  {item.downloaded ? (
                    <button
                      className="row-span-2 mt-1 focus:outline-none text-green-300 cursor-pointer"
                      onClick={() => handleDelete(item.id)}
                    >
                      <CircleCheckIcon />
                    </button>
                  ) : (
                    <button
                      className="row-span-2 mt-1 opacity-70 focus:outline-none cursor-pointer"
                      onClick={() => handleDownload(item.id)}
                    >
                      <DownloadIcon />
                    </button>
                  )}
                  <div>{item.name}</div>
                  <div className="opacity-70">{item.albumArtist}</div>
                </div>
              </li>
            ))}
          </ul>

          <div className="flex justify-between mt-4">
            <button
              onClick={() => executeSearch(Math.max(0, offset - limit))}
              disabled={offset === 0}
              className="disabled:hidden"
            >
              Previous
            </button>
            <button
              onClick={() => executeSearch(offset + limit)}
              disabled={results.items.length < limit}
              className="disabled:hidden"
            >
              Next
            </button>
          </div>
        </div>
      ) : null}
    </main>
  );
}

export default HomePage;
