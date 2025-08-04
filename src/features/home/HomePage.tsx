import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AlbumSearchResponse } from "../auth/types";
import Logo from "./Logo";
import { useDownloadStatus } from "./useDownloadStatus";
import { useFocusOnKeyPress } from "./useFocusOnKeyPress";
import DownloadIcon from "./DownloadIcon";
import CircleCheckIcon from "./CircleCheckIcon";
import { useQuery, useQueryClient } from "@tanstack/react-query";

const limit = 50;

function HomePage() {
  const [search, setSearch] = useState("");
  const [offset, setOffset] = useState(0);
  const searchInputRef = useRef<HTMLInputElement>(null);
  const resultsRef = useRef<HTMLUListElement>(null);
  const isDownloading = useDownloadStatus();
  const [focusedAlbumId, setFocusedAlbumId] = useState<string | null>(null);
  const queryClient = useQueryClient();

  const {
    data: results,
    isFetching: isSearching,
    refetch,
  } = useQuery({
    queryKey: ["search_albums", search, limit, offset],
    queryFn: () =>
      invoke<AlbumSearchResponse>("search_albums", { search, limit, offset }),
    enabled: false,
  });

  useFocusOnKeyPress("/", searchInputRef);

  useEffect(() => {
    if (focusedAlbumId && resultsRef.current) {
      const liElement = resultsRef.current.querySelector(
        `li[data-album-id='${focusedAlbumId}']`
      );

      const buttonToFocus = liElement?.querySelector("button");
      buttonToFocus?.focus();
    }
  }, [results, focusedAlbumId]);

  useEffect(() => {
    refetch();
  }, [offset, refetch]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const { data } = await refetch();
    if (data && data.items.length > 0) {
      setFocusedAlbumId(data.items[0].id);
    }
  };

  const setDownloaded = (id: string, downloaded: boolean) => {
    queryClient.setQueryData<AlbumSearchResponse | undefined>(
      ["search_albums", search, limit, offset],
      (prev) => {
        if (!prev) {
          return prev;
        }

        return {
          ...prev,
          items: prev.items.map((item) =>
            item.id === id ? { ...item, downloaded } : item
          ),
        };
      }
    );
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
                      className="row-span-2 mt-1 focus:outline-none cursor-pointer"
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
              onClick={() => setOffset((prev) => Math.max(0, prev - limit))}
              disabled={offset === 0}
            >
              Previous
            </button>
            <button
              onClick={() => setOffset((prev) => prev + limit)}
              disabled={results.items.length < limit}
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
