import Controls from "../playback/Controls";
import { usePlayback } from "../playback/PlaybackProvider";

function MiniPlayer() {
  const { album, track } = usePlayback();

  if (!album || !track) {
    return null;
  }

  return (
    <div className="flex justify-between items-center gap-2">
      <div>
        <div className="text-sm">{track.name}</div>
        <div className="text-zinc-400 text-xs">{album.artist}</div>
      </div>
      <Controls size="sm" />
    </div>
  );
}

export default MiniPlayer;
