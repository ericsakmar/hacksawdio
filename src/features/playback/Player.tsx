import LeftArrowIcon from "../components/LeftArrowIcon";
import PauseCircleIcon from "../components/PauseCircleIcon";
import PlayCircleIcon from "../components/PlayCircleIcon";
import PlayIcon from "../components/PlayIcon";
import RightArrowIcon from "../components/RightArrowIcon";
import { usePlayback } from "./PlaybackProvider";

function Player() {
  const {
    album,
    track,
    isPlaying,
    duration,
    currentTime,
    togglePlayPause,
    handleSeek,
    handleNextTrack,
    hasNextTrack,
    handlePreviousTrack,
    hasPreviousTrack,
  } = usePlayback();

  if (!album || !track) {
    return <div className="player">No album selected</div>;
  }

  return (
    <div>
      <div className="flex items-center justify-center mb-2 gap-2">
        <button
          onClick={handlePreviousTrack}
          disabled={!hasPreviousTrack}
          className="disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <LeftArrowIcon />
        </button>

        <button
          onClick={togglePlayPause}
          className="disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isPlaying ? (
            <PauseCircleIcon className="w-12 h-12 text-blue-500" />
          ) : (
            <PlayCircleIcon className="w-12 h-12" />
          )}
        </button>

        <button
          onClick={handleNextTrack}
          disabled={!hasNextTrack}
          className="disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <RightArrowIcon />
        </button>
      </div>

      <div className="text-center">
        <h2>{track.name}</h2>
        <p className="text-sm text-gray-500">
          from {album.name} by {album.artist}
        </p>
      </div>

      <div className="flex items-center justify-center space-x-2 my-2">
        <span className="text-xs">{`${Math.floor(
          currentTime / 60
        )}:${Math.floor(currentTime % 60)
          .toString()
          .padStart(2, "0")}`}</span>

        <input
          type="range"
          min="0"
          max={duration}
          value={currentTime}
          onChange={(e) => handleSeek(parseFloat(e.target.value))}
          step="0.01"
        />

        <span className="text-xs">{`${Math.floor(duration / 60)}:${Math.floor(
          duration % 60
        )
          .toString()
          .padStart(2, "0")}`}</span>
      </div>
    </div>
  );
}

export default Player;
