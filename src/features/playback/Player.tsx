import { useHotkeys } from "react-hotkeys-hook";
import { convertFileSrc } from "@tauri-apps/api/core";
import SpeakerIcon from "../components/SpeakerIcon";
import Controls from "./Controls";
import { usePlayback } from "./PlaybackProvider";
import Seeker from "./Seeker";

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
        className="relative mb-8 aspect-square w-full overflow-hidden rounded border-2 border-dashed border-zinc-600 bg-zinc-900 px-8 py-4 shadow-black shadow"
        style={{
          backgroundImage: albumArt
            ? `linear-gradient(rgba(24,24,27,0.90), rgba(24,24,27,0.95)), url(${albumArt})`
            : "none",
          backgroundSize: "cover",
          backgroundPosition: "center",
        }}
      >
        <div className="flex h-full flex-col justify-center">
          <h1 className="text-center">{album.name}</h1>
          <h2 className="mb-4 text-center text-sm font-light text-zinc-400">
            {album.artist}
          </h2>

          <Controls />

          <h2 className="mt-4 text-center text-lg">
            {trackIndex! + 1}. {track.name}
          </h2>

          <Seeker />
        </div>
      </div>

      <div>
        {tracks.map((t, index) => {
          const isActive = index === trackIndex;

          return (
            <button
              key={index}
              type="button"
              onClick={() => handleTrackSelect(index)}
              className={`flex w-full cursor-pointer items-baseline gap-2 border-l-2 px-3 py-2.5 text-left ${
                isActive
                  ? "border-amber-300 bg-zinc-900 text-amber-300"
                  : "border-transparent text-zinc-100"
              } ${index > 0 ? "border-t border-t-zinc-700/50" : ""}`}
            >
              <span
                className={`w-8 shrink-0 tabular-nums ${
                  isActive ? "text-amber-300/80" : "text-zinc-500"
                }`}
              >
                {index + 1}
              </span>
              <span className="min-w-0 flex-1">{t.name}</span>
              {isActive ? (
                <SpeakerIcon className="h-4 w-4 shrink-0 self-center opacity-70" />
              ) : null}
            </button>
          );
        })}
      </div>
    </div>
  );
}

export default Player;
