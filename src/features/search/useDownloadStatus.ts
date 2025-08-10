import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

export function useDownloadStatus() {
  const [isQueueActive, setIsQueueActive] = useState(false);

  useEffect(() => {
    let unlistenAlbumDownloadStarted: () => void;
    let unlisetnAlbumDownloadFinished: () => void;
    // albumDownloadError?

    const setupListeners = async () => {
      unlistenAlbumDownloadStarted = await listen<void>(
        "album-download-started",
        (event) => {
          console.log("Album download started:", event);
          setIsQueueActive(true);
        }
      );

      unlisetnAlbumDownloadFinished = await listen<void>(
        "album-download-completed",
        (event) => {
          console.log("Download queue is now empty:", event);
          setIsQueueActive(false);
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

  return isQueueActive;
}
