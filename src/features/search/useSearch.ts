import { useEffect, useState } from "react";
import { AlbumSearchResponse } from "../auth/types";
import { invoke } from "@tauri-apps/api/core";

export type ResultsState = AlbumSearchResponse & {
  focusedAlbumId: string | null;
};

export type OfflineView = "recent" | "byArtist";

const limit = 50;

function getSummary(
  search: string,
  resultCount: number,
  limit: number,
  offset: number,
  offlineView: OfflineView
) {
  if (search === "") {
    return offlineView === "byArtist" ? "By artist" : "Recently added";
  }

  if (resultCount < limit) {
    return `${resultCount} albums`;
  }

  return `${offset + 1} to ${Math.min(
    offset + limit,
    resultCount
  )} of ${resultCount} albums`;
}

export function useSearch(isOnline: boolean) {
  const [search, setSearch] = useState("");
  const [offlineView, setOfflineView] = useState<OfflineView>("recent");
  const [offset, setOffset] = useState(0);
  const [results, setResults] = useState<ResultsState | null>(null);
  const [isSearching, setIsSearching] = useState(false);
  const [summary, setSummary] = useState("");

  // search again when changing between online and offline mode
  useEffect(() => {
    executeSearch(0);
  }, [isOnline, offlineView]);

  const executeSearch = async (newOffset: number) => {
    setIsSearching(true);

    const res = await invoke<AlbumSearchResponse>("search_albums", {
      search,
      limit,
      offset: newOffset,
      online: isOnline,
      offlineView: !isOnline && search === "" ? offlineView : null,
    });

    setSummary(
      getSummary(search, res.totalRecordCount, limit, newOffset, offlineView)
    );
    setResults({
      ...res,
      focusedAlbumId: res.items.length > 0 ? res.items[0].id : null,
    });
    setOffset(newOffset);
    setIsSearching(false);
  };

  const setDownloaded = (id: string, downloaded: boolean, remove: boolean) => {
    if (remove) {
      setResults((prev) => {
        if (!prev) {
          return prev;
        }

        return {
          ...prev,
          items: prev.items.filter((item) => item.id !== id),
        };
      });
    } else {
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
    }
  };

  const setFocusedAlbumId = (id: string | null) => {
    setResults((prev) => {
      if (!prev) {
        return prev;
      }

      return {
        ...prev,
        focusedAlbumId: id,
      };
    });
  };

  return {
    search,
    setSearch,
    offlineView,
    setOfflineView,
    results,
    isSearching,
    executeSearch,
    setDownloaded,
    offset,
    setOffset,
    limit,
    summary,
    setFocusedAlbumId,
  };
}
