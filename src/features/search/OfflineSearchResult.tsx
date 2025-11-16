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
}

function OfflineSearchResult({ item, handleDelete, handlePlay }: Props) {
  const { isAlbumDownloading } = useDownloadStatus();
  const isDownloading = isAlbumDownloading(item.id);
  const albumArt = item.imageUrl ? convertFileSrc(item.imageUrl) : null;

  return (
    <div className="group grid grid-cols-[auto_1fr_auto] gap-x-2 items-start focus-within:bg-zinc-900 rounded p-2 hover:bg-zinc-900">
      <button
        className="w-16 h-16 cursor-pointer flex items-center justify-center rounded "
        onClick={() => handlePlay(item.id)}
        aria-label={isDownloading ? "Album is downloading" : "Play album"}
        style={{
          backgroundImage: albumArt ? `url(${albumArt})` : "none",
          backgroundSize: "cover",
          backgroundPosition: "center",
        }}
      >
        <PlayIcon className="w-8 h-8" />
      </button>

      <div>
        <div>{item.name}</div>
        <div className="opacity-70 font-light">{item.albumArtist}</div>
      </div>

      {isDownloading ? null : (
        <button
          className="row-span-2 opacity-0 group-hover:opacity-70 peer-focus:opacity-70 focus:opacity-100 focus:outline-none cursor-pointer hover:text-red-500 hover:opacity-100 focus:text-red-500"
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
