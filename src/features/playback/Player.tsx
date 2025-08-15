import { useHotkeys } from "react-hotkeys-hook";
import Controls from "./Controls";
import { usePlayback } from "./PlaybackProvider";
import Seeker from "./Seeker";
import { convertFileSrc } from "@tauri-apps/api/core";

function Player() {
  const { album, track, tracks, trackIndex, setTrackIndex, togglePlayPause } =
    usePlayback();

  // j to select the next track
  useHotkeys("j", () => {
    if (trackIndex !== null && trackIndex < tracks.length - 1) {
      setTrackIndex(trackIndex + 1);
    }
  });

  // k to select the previous track
  useHotkeys("k", () => {
    if (trackIndex !== null && trackIndex > 0) {
      setTrackIndex(trackIndex - 1);
    }
  });

  // space to toggle play/pause
  useHotkeys("space", (event) => {
    event.preventDefault();
    togglePlayPause();
  });

  const handleTrackSelect = (index: number) => {
    setTrackIndex(index);
  };

  if (!album || !track) {
    return <div className="text-center text-2xl">No album selected</div>;
  }

  const albumArt = album.imageUrl ? convertFileSrc(album.imageUrl) : null;

  return (
    <div>
      <div
        className={`bg-zinc-900 rounded py-4 px-8 mb-8 border-zinc-600 border-dashed border-2 shadow-black shadow`}
        style={{
          backgroundImage: albumArt
            ? `linear-gradient(rgba(24,24,27,0.90), rgba(24,24,27,0.95)), url(${albumArt})`
            : "none",
          backgroundSize: "cover",
          backgroundPosition: "center",
        }}
      >
        <h1 className="text-center">{album.name}</h1>
        <h2 className="text-zinc-400 text-sm mb-4 text-center font-light">
          {album.artist}
        </h2>

        <Controls />

        <h2 className="text-center text-lg mt-4">
          {trackIndex! + 1}. {track.name}
        </h2>

        <Seeker />
      </div>

      <div>
        {tracks.map((t, index) => (
          <div
            key={index}
            className={`px-4 py-2 cursor-pointer flex gap-1 ${
              index === trackIndex
                ? "py-4 bg-zinc-900 rounded text-amber-300"
                : ""
            }`}
            onClick={() => handleTrackSelect(index)}
          >
            <span className="">{index + 1}.</span> <span>{t.name}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

export default Player;
