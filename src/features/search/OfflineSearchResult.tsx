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

  return (
    <div className="group grid grid-cols-[auto_1fr_auto] gap-x-2 items-start focus-within:bg-zinc-900 rounded p-2 hover:bg-zinc-900">
      <ActionButton
        className="peer row-span-2 mt-1 opacity-70 focus:opacity-100 focus:outline-none focus:text-green-300 cursor-pointer"
        onClick={() => handlePlay(item.id)}
        isLoading={isDownloading}
        ariaLabel={isDownloading ? "Album is downloading" : "Play album"}
      >
        <PlayIcon className="w-4 h-4" />
      </ActionButton>

      <div>{item.name}</div>

      {isDownloading ? null : (
        <button
          className="row-span-2 opacity-0 group-hover:opacity-70 peer-focus:opacity-70 focus:opacity-100 focus:outline-none cursor-pointer hover:text-red-500 hover:opacity-100 focus:text-red-500"
          onClick={() => handleDelete(item.id)}
          aria-label="Delete album from downloads"
        >
          <DeleteIcon />
        </button>
      )}

      <div className="opacity-70">{item.albumArtist}</div>
    </div>
  );
}

export default OfflineSearchResult;
