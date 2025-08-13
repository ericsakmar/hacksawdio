import { useHotkeys } from "react-hotkeys-hook";
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

  return (
    <div>
      <div className="bg-zinc-900 rounded py-4 px-8 mb-8 border-zinc-600 border-dashed border-2 shadow-black shadow">
        <h1 className="text-center">{album.name}</h1>
        <h2 className="text-zinc-400 text-sm mb-4 text-center">
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
            className={`px-4 py-2 cursor-pointer ${
              index === trackIndex ? "bg-zinc-900 rounded" : ""
            }`}
            onClick={() => handleTrackSelect(index)}
          >
            {index + 1}. {t.name}
          </div>
        ))}
      </div>
    </div>
  );
}

export default Player;
