import { convertFileSrc } from "@tauri-apps/api/core";
import { AlbumSearchResponseItem } from "../auth/types";
import ActionButton from "../components/ActionButton";
import DeleteIcon from "../components/DeleteIcon";
import PlayIcon from "../components/PlayIcon";
import { useDownloadStatus } from "./useDownloadStatus";

interface Props {
  item: AlbumSearchResponseItem;
  handleDelete: (id: string) => void;
  handlePlay: (id: string) => void;
  showArtist?: boolean;
  variant?: "list" | "card";
}

function AlbumArt({
  imageUrl,
  className,
}: {
  imageUrl?: string;
  className: string;
}) {
  if (imageUrl) {
    return (
      <img
        src={convertFileSrc(imageUrl)}
        alt=""
        className={`object-cover ${className}`}
      />
    );
  }

  return (
    <div
      className={`flex items-center justify-center bg-zinc-800 text-zinc-500 ${className}`}
    >
      <PlayIcon className="w-5 h-5" />
    </div>
  );
}

function OfflineSearchResult({
  item,
  handleDelete,
  handlePlay,
  showArtist = true,
  variant = "list",
}: Props) {
  const { isAlbumDownloading } = useDownloadStatus();
  const isDownloading = isAlbumDownloading(item.id);
  const albumArt = item.imageUrl ? convertFileSrc(item.imageUrl) : null;

  if (variant === "card") {
    return (
      <div
        className="group relative flex flex-col rounded bg-zinc-900 focus-within:ring-1 focus-within:ring-amber-300/60"
        data-album-id={item.id}
      >
        <button
          type="button"
          onClick={() => handlePlay(item.id)}
          disabled={isDownloading}
          className="flex w-full flex-col text-left focus:outline-none cursor-pointer disabled:cursor-wait"
          aria-label={
            isDownloading ? "Album is downloading" : `Play ${item.name}`
          }
        >
          <div className="relative aspect-square overflow-hidden rounded-t">
            <AlbumArt imageUrl={item.imageUrl} className="w-full h-full" />
            <div className="absolute inset-0 flex items-center justify-center bg-black/40 opacity-0 transition-opacity group-hover:opacity-100 group-focus-within:opacity-100">
              {isDownloading ? null : (
                <PlayIcon className="w-8 h-8 text-zinc-100" />
              )}
            </div>
          </div>
          <div className="p-2 text-sm leading-snug break-words">
            {item.name}
          </div>
        </button>

        {isDownloading ? null : (
          <button
            type="button"
            className="absolute top-2 right-2 rounded bg-zinc-950/80 p-1 opacity-0 transition-opacity group-hover:opacity-100 focus:opacity-100 focus:outline-none cursor-pointer hover:text-red-400"
            onClick={() => handleDelete(item.id)}
            aria-label="Delete album from downloads"
          >
            <DeleteIcon />
          </button>
        )}
      </div>
    );
  }

  return (
    <div className="group grid grid-cols-[auto_1fr_auto] gap-x-3 items-center focus-within:bg-zinc-900 rounded p-2 hover:bg-zinc-900">
      <ActionButton
        isLoading={isDownloading}
        className="relative shrink-0 overflow-hidden rounded focus:outline-none focus:ring-1 focus:ring-amber-300/60 cursor-pointer"
        onClick={() => handlePlay(item.id)}
        aria-label={isDownloading ? "Album is downloading" : "Play album"}
      >
        <AlbumArt imageUrl={item.imageUrl} className="w-12 h-12" />
        <div className="absolute inset-0 flex items-center justify-center bg-black/40 opacity-0 transition-opacity group-hover:opacity-100 group-focus-within:opacity-100">
          {isDownloading ? null : (
            <PlayIcon className="w-4 h-4 text-zinc-100" />
          )}
        </div>
      </ActionButton>

      <div className="min-w-0">
        <div className="line-clamp-1">{item.name}</div>
        {showArtist ? (
          <div className="opacity-70 font-light text-sm line-clamp-1">
            {item.albumArtist}
          </div>
        ) : null}
      </div>

      {isDownloading ? null : (
        <button
          className="opacity-0 group-hover:opacity-70 focus:opacity-100 focus:outline-none cursor-pointer hover:text-red-500 hover:opacity-100 focus:text-red-500"
          onClick={() => handleDelete(item.id)}
          aria-label="Delete album from downloads"
        >
          <DeleteIcon />
        </button>
      )}
    </div>
  );
}

export default OfflineSearchResult;
