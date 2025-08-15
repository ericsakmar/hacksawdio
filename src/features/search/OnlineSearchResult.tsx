import { AlbumSearchResponseItem } from "../auth/types";
import ActionButton from "../components/ActionButton";
import CircleCheckIcon from "../components/CircleCheckIcon";
import DownloadIcon from "../components/DownloadIcon";
import { useDownloadStatus } from "./useDownloadStatus";

interface Props {
  item: AlbumSearchResponseItem;
  handleDownload: (id: string) => void;
  handleDelete: (id: string) => void;
}

function OnlineSearchResult({ item, handleDelete, handleDownload }: Props) {
  const { isAlbumDownloading } = useDownloadStatus();
  const isDownloading = isAlbumDownloading(item.id);
  const downloaded = item.downloaded && !isDownloading;

  return (
    <div className="grid grid-cols-[auto_1fr] gap-x-2 items-start hover:bg-zinc-900 focus-within:bg-zinc-900 rounded p-2">
      {downloaded ? (
        <ActionButton
          className="row-span-2 mt-1 focus:outline-none text-green-300 cursor-pointer"
          onClick={() => handleDelete(item.id)}
          isLoading={false}
          ariaLabel="Delete album from downloads"
        >
          <CircleCheckIcon />
        </ActionButton>
      ) : (
        <ActionButton
          className="row-span-2 mt-1 opacity-70 focus:outline-none cursor-pointer"
          onClick={() => handleDownload(item.id)}
          isLoading={isDownloading}
          ariaLabel={isDownloading ? "Downloading album" : "Download album"}
        >
          <DownloadIcon />
        </ActionButton>
      )}
      <div>{item.name}</div>
      <div className="opacity-70 font-light">{item.albumArtist}</div>
    </div>
  );
}

export default OnlineSearchResult;
