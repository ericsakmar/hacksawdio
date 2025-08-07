import { convertFileSrc } from "@tauri-apps/api/core";
import { usePlayback } from "./PlaybackProvider";

function Player() {
  const { album, track } = usePlayback();

  if (!album || !track) {
    return <div className="player">No album selected</div>;
  }

  const src = convertFileSrc(track.playbackUrl);

  return (
    <div>
      <div className="text-center">
        <h2>{track.name}</h2>
        <p className="text-sm text-gray-500">
          from {album.name} by {album.artist}
        </p>
      </div>

      <audio controls className="w-full mt-4" src={src} />
    </div>
  );
}

export default Player;
