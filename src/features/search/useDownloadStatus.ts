import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

export function useDownloadStatus() {
  const [isQueueActive, setIsQueueActive] = useState(false);

  useEffect(() => {
    let unlistenNotEmpty: () => void;
    let unlistenEmpty: () => void;

    const setupListeners = async () => {
      // Listen for the 'download-queue-not-empty' event
      unlistenNotEmpty = await listen<void>(
        "download-queue-not-empty",
        (event) => {
          console.log("Download queue is now active:", event);
          setIsQueueActive(true);
        }
      );

      // Listen for the 'download-queue-empty' event
      unlistenEmpty = await listen<void>("download-queue-empty", (event) => {
        console.log("Download queue is now empty:", event);
        setIsQueueActive(false);
      });
    };

    setupListeners();

    // Cleanup function: This will be called when the component unmounts.
    return () => {
      if (unlistenNotEmpty) {
        unlistenNotEmpty();
      }
      if (unlistenEmpty) {
        unlistenEmpty();
      }
    };
  }, []); // The empty dependency array ensures this effect runs only once on mount.

  return isQueueActive;
}
