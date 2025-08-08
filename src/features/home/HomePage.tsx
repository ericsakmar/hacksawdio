import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Logo from "../components/Logo";
import { useDownloadStatus } from "./useDownloadStatus";
import { useSearch } from "./useSearch";
import OnlineIcon from "../components/OnlineIcon";
import OfflineIcon from "../components/OfflineIcon";
import { useHomeHotkeys } from "./useHomeHotkeys";
import OnlineSearchResult from "./OnlineSearchResult";
import OfflineSearchResult from "./OfflineSearchResult";
import { usePlayback } from "../playback/PlaybackProvider";
import Nav from "../Nav";
import { useNavigate } from "react-router";
import MiniPlayer from "./MiniPlayer";

function HomePage() {
  const searchInputRef = useRef<HTMLInputElement>(null);
  const resultsRef = useRef<HTMLUListElement>(null);
  const isDownloading = useDownloadStatus();
  const [focusedAlbumId, setFocusedAlbumId] = useState<string | null>(null);
  const [isOnline, setIsOnline] = useState(false);
  const { setAlbum, album } = usePlayback();
  const navigate = useNavigate();

  const {
    executeSearch,
    isSearching,
    limit,
    offset,
    results,
    search,
    setDownloaded,
    setSearch,
    summary,
  } = useSearch(isOnline);

  useHomeHotkeys({
    executeSearch,
    focusedAlbumId,
    limit,
    offset,
    results,
    searchInputRef,
    setFocusedAlbumId,
    setIsOnline,
  });

  // keeps focus between searches and sets initial focus
  useEffect(() => {
    if (results && results.items.length > 0) {
      if (focusedAlbumId === null) {
        setFocusedAlbumId(results.items[0].id);
      }
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
    setFocusedAlbumId(null);
    executeSearch(0);
  };

  const handleDownload = async (id: string) => {
    await invoke("download_album", { albumId: id });
    setDownloaded(id, true, false);
    setFocusedAlbumId(id);
  };

  const handleDelete = async (id: string) => {
    await invoke("delete_album", { albumId: id });

    if (isOnline) {
      setDownloaded(id, false, false);
      setFocusedAlbumId(id);
    } else {
      setDownloaded(id, false, true);
      setFocusedAlbumId(null);
    }
  };

  const handleOnlineToggle = () => {
    setIsOnline((prev) => !prev);
  };

  const handlePlay = async (id: string) => {
    const album = await invoke<Album>("get_album_info", { albumId: id });
    setAlbum(album);
    navigate("/player");
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
        <Nav />
      </header>

      <form
        onSubmit={handleSubmit}
        className="bg-zinc-900 border-zinc-600 border-dashed border-2 p-4 my-4 flex rounded shadow-black shadow gap-4 focus-within:border-amber-300"
      >
        <input
          type="search"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search for an artist or album"
          className="flex-grow focus:outline-none"
          ref={searchInputRef}
        />

        <button type="submit" className="focus:outline-blue-500">
          Search
        </button>
      </form>

      <p className="mb-4 ml-2 opacity-70">{summary}</p>

      {results ? (
        <div className="mb-4">
          <ul ref={resultsRef} className="">
            {results.items.map((item) => (
              <li key={item.id} data-album-id={item.id}>
                {isOnline ? (
                  <OnlineSearchResult
                    item={item}
                    handleDelete={handleDelete}
                    handleDownload={handleDownload}
                  />
                ) : (
                  <OfflineSearchResult
                    item={item}
                    handleDelete={handleDelete}
                    handlePlay={handlePlay}
                  />
                )}
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

      {album ? (
        <div className="fixed bottom-0 left-0 right-0 bg-zinc-900 px-4 py-2">
          <MiniPlayer />
        </div>
      ) : null}
    </main>
  );
}

export default HomePage;
