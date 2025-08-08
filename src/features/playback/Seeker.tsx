import { usePlayback } from "./PlaybackProvider";

function Seeker() {
  const { duration, currentTime, handleSeek } = usePlayback();

  return (
    <div className="flex items-center justify-center space-x-2 my-2">
      <span className="text-xs">{`${Math.floor(currentTime / 60)}:${Math.floor(
        currentTime % 60
      )
        .toString()
        .padStart(2, "0")}`}</span>

      <input
        type="range"
        min="0"
        max={duration}
        value={currentTime}
        onChange={(e) => handleSeek(parseFloat(e.target.value))}
        step="0.01"
        className="basis-lg"
      />

      <span className="text-xs">{`${Math.floor(duration / 60)}:${Math.floor(
        duration % 60
      )
        .toString()
        .padStart(2, "0")}`}</span>
    </div>
  );
}

export default Seeker;
