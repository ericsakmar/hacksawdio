import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

interface DownloadStatus {
  album_id: string;
}

export function useDownloadStatus() {
  const [isQueueActive, setIsQueueActive] = useState(false);
  const [currentlyDownloading, setCurrentlyDownloading] = useState<Set<string>>(
    new Set()
  );

  useEffect(() => {
    if (currentlyDownloading.size > 0) {
      setIsQueueActive(true);
    } else {
      setIsQueueActive(false);
    }
  }, [currentlyDownloading]);

  useEffect(() => {
    let unlistenAlbumDownloadStarted: () => void;
    let unlisetnAlbumDownloadFinished: () => void;
    // albumDownloadError?

    const setupListeners = async () => {
      unlistenAlbumDownloadStarted = await listen<DownloadStatus>(
        "album-download-started",
        (event) => {
          console.log("Album download started:", event);
          setCurrentlyDownloading((prev) => {
            const newSet = new Set(prev);
            newSet.add(event.payload.album_id);
            return newSet;
          });
        }
      );

      unlisetnAlbumDownloadFinished = await listen<DownloadStatus>(
        "album-download-completed",
        (event) => {
          console.log("Album download finished", event);
          setCurrentlyDownloading((prev) => {
            const newSet = new Set(prev);
            newSet.delete(event.payload.album_id);
            return newSet;
          });
        }
      );
    };

    setupListeners();

    return () => {
      if (unlistenAlbumDownloadStarted) {
        unlistenAlbumDownloadStarted();
      }
      if (unlisetnAlbumDownloadFinished) {
        unlisetnAlbumDownloadFinished();
      }
    };
  }, []); // The empty dependency array ensures this effect runs only once on mount.

  const isAlbumDownloading = (albumId: string): boolean => {
    return currentlyDownloading.has(albumId);
  };

  return { isQueueActive, isAlbumDownloading };
}
