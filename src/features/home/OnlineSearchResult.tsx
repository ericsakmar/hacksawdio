import { AlbumSearchResponseItem } from "../auth/types";
import CircleCheckIcon from "../components/CircleCheckIcon";
import DownloadIcon from "../components/DownloadIcon";

interface Props {
  item: AlbumSearchResponseItem;
  handleDownload: (id: string) => void;
  handleDelete: (id: string) => void;
}

function OnlineSearchResult({ item, handleDelete, handleDownload }: Props) {
  return (
    <div className="grid grid-cols-[auto_1fr] gap-x-2 items-start focus-within:bg-zinc-900 rounded p-2">
      {item.downloaded ? (
        <button
          className="row-span-2 mt-1 focus:outline-none text-green-300 cursor-pointer"
          onClick={() => handleDelete(item.id)}
        >
          <CircleCheckIcon />
        </button>
      ) : (
        <button
          className="row-span-2 mt-1 opacity-70 focus:outline-none cursor-pointer"
          onClick={() => handleDownload(item.id)}
        >
          <DownloadIcon />
        </button>
      )}
      <div>{item.name}</div>
      <div className="opacity-70">{item.albumArtist}</div>
    </div>
  );
}

export default OnlineSearchResult;
