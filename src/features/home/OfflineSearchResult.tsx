import { AlbumSearchResponseItem } from "../auth/types";
import DeleteIcon from "../components/DeleteIcon";
import PlayIcon from "../components/PlayIcon";

interface Props {
  item: AlbumSearchResponseItem;
  handleDelete: (id: string) => void;
  handlePlay: (id: string) => void;
}

function OfflineSearchResult({ item, handleDelete, handlePlay }: Props) {
  return (
    <div className="grid grid-cols-[auto_1fr_auto] gap-x-2 items-start focus-within:bg-zinc-900 rounded p-2">
      <button
        className="peer row-span-2 mt-1 opacity-70 focus:opacity-100 focus:outline-none focus:text-green-300 cursor-pointer"
        onClick={() => handlePlay(item.id)}
      >
        <PlayIcon className="w-4 h-4" />
      </button>

      <div>{item.name}</div>

      <button
        className="row-span-2 opacity-0 peer-focus:opacity-70 focus:opacity-100 focus:outline-none cursor-pointer hover:text-red-500 focus:text-red-500"
        onClick={() => handleDelete(item.id)}
      >
        <DeleteIcon />
      </button>

      <div className="opacity-70">{item.albumArtist}</div>
    </div>
  );
}

export default OfflineSearchResult;
