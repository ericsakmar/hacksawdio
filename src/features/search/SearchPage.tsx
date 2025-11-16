import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSearch } from "./useSearch";
import { useSearchHotkeys } from "./useSearchHotkeys";
import OnlineSearchResult from "./OnlineSearchResult";
import OfflineSearchResult from "./OfflineSearchResult";
import { usePlayback } from "../playback/PlaybackProvider";
import { useNavigate } from "react-router";
import MiniPlayer from "./MiniPlayer";
import { useOnlineStatus } from "../OnlineStatusProvider";

function SearchPage() {
  const searchInputRef = useRef<HTMLInputElement>(null);
  const resultsRef = useRef<HTMLUListElement>(null);
  const { isOnline } = useOnlineStatus();
  const { setAlbum, album } = usePlayback();
  const navigate = useNavigate();

  const {
    executeSearch,
    limit,
    offset,
    results,
    search,
    setDownloaded,
    setSearch,
    summary,
    setFocusedAlbumId,
  } = useSearch(isOnline);

  useSearchHotkeys({
    executeSearch,
    limit,
    offset,
    results,
    searchInputRef,
    setFocusedAlbumId,
  });

  // sets the focus
  const focusedAlbumId = results?.focusedAlbumId || null;
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

  const handlePlay = async (id: string) => {
    const album = await invoke<Album>("get_album_info", { albumId: id });
    setAlbum(album);
    navigate("/player");
  };

  return (
    <>
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
          {isOnline ? (
            <ul ref={resultsRef} className="">
              {results.items.map((item) => (
                <li key={item.id} data-album-id={item.id}>
                  <OnlineSearchResult
                    item={item}
                    handleDelete={handleDelete}
                    handleDownload={handleDownload}
                  />
                </li>
              ))}
            </ul>
          ) : (
            <ul ref={resultsRef} className="">
              {results.items.map((item) => (
                <li key={item.id} data-album-id={item.id}>
                  <OfflineSearchResult
                    item={item}
                    handleDelete={handleDelete}
                    handlePlay={handlePlay}
                  />
                </li>
              ))}
            </ul>
          )}

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
        <div className="fixed bottom-0 left-0 right-0 bg-zinc-900 px-4 py-4">
          <MiniPlayer />
        </div>
      ) : null}
    </>
  );
}

export default SearchPage;
