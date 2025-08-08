import LeftArrowIcon from "../components/LeftArrowIcon";
import PauseCircleIcon from "../components/PauseCircleIcon";
import PlayCircleIcon from "../components/PlayCircleIcon";
import RightArrowIcon from "../components/RightArrowIcon";
import { usePlayback } from "./PlaybackProvider";

interface Props {
  size?: "sm" | "md";
}

function Controls({ size = "md" }: Props) {
  const {
    isPlaying,
    togglePlayPause,
    handleNextTrack,
    hasNextTrack,
    handlePreviousTrack,
    hasPreviousTrack,
  } = usePlayback();

  return (
    <div className="flex items-center justify-center gap-2">
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
          <PauseCircleIcon
            className={`${
              size === "sm" ? "w-8 h-8" : "w-12 h-12"
            } text-blue-500`}
          />
        ) : (
          <PlayCircleIcon className={size === "sm" ? "w-8 h-8" : "w-12 h-12"} />
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
  );
}

export default Controls;
