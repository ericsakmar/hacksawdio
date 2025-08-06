import { RefObject } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { SearchResults } from "./useSearch";

interface UseHomeHotkeysProps {
  executeSearch: (newOffset: number) => void;
  focusedAlbumId: string | null;
  limit: number;
  offset: number;
  results: SearchResults | null;
  searchInputRef: RefObject<HTMLInputElement>;
  setFocusedAlbumId: (id: string) => void;
  setIsOnline: (updater: (prev: boolean) => boolean) => void;
}

export const useHomeHotkeys = ({
  executeSearch,
  focusedAlbumId,
  limit,
  offset,
  results,
  searchInputRef,
  setFocusedAlbumId,
  setIsOnline,
}: UseHomeHotkeysProps) => {
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

  // move to the next search result with "j"
  useHotkeys(
    "j",
    () => {
      if (!results || results.items.length === 0) {
        return;
      }

      const currentIndex = results.items.findIndex(
        (item) => item.id === focusedAlbumId
      );

      if (currentIndex < results.items.length - 1) {
        setFocusedAlbumId(results.items[currentIndex + 1].id);
      }
    },
    {
      preventDefault: true,
    }
  );

  // move to the previous search result with "k"
  useHotkeys(
    "k",
    () => {
      if (!results || results.items.length === 0) {
        return;
      }

      const currentIndex = results.items.findIndex(
        (item) => item.id === focusedAlbumId
      );

      if (currentIndex > 0) {
        setFocusedAlbumId(results.items[currentIndex - 1].id);
      }
    },
    {
      preventDefault: true,
    }
  );

  // go to previous page with "h"
  useHotkeys(
    "h",
    () => {
      if (offset > 0) {
        executeSearch(Math.max(0, offset - limit));
      }
    },
    {
      preventDefault: true,
    }
  );

  // go to next page with "l"
  useHotkeys(
    "l",
    () => {
      if (results && results.items.length >= limit) {
        executeSearch(offset + limit);
      }
    },
    {
      preventDefault: true,
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
};