import { useState } from "react";
import { AlbumSearchResponse } from "../auth/types";
import { invoke } from "@tauri-apps/api/core";

const limit = 50;

export function useSearch() {
  const [search, setSearch] = useState("");
  const [offset, setOffset] = useState(0);
  const [results, setResults] = useState<AlbumSearchResponse | null>(null);
  const [isSearching, setIsSearching] = useState(false);

  const executeSearch = async (newOffset: number) => {
    setIsSearching(true);

    const res = await invoke<AlbumSearchResponse>("search_albums", {
      search,
      limit,
      offset: newOffset,
    });

    setResults(res);
    setOffset(newOffset);
    setIsSearching(false);
  };

  const setDownloaded = (id: string, downloaded: boolean) => {
    setResults((prev) => {
      if (!prev) {
        return prev;
      }

      return {
        ...prev,
        items: prev.items.map((item) =>
          item.id === id ? { ...item, downloaded } : item
        ),
      };
    });
  };

  return {
    search,
    setSearch,
    results,
    isSearching,
    executeSearch,
    setDownloaded,
    offset,
    setOffset,
    limit,
  };
}
