import Controls from "./Controls";
import { usePlayback } from "./PlaybackProvider";
import Seeker from "./Seeker";

function Player() {
  const { album, track, tracks, trackIndex } = usePlayback();

  if (!album || !track) {
    return <div className="text-center text-2xl">No album selected</div>;
  }

  return (
    <div>
      <div className="bg-zinc-900 rounded-lg py-4 px-8 mb-8">
        <h1 className="text-2xl text-center">{album.name}</h1>
        <h2 className="text-zinc-400 mb-4 text-center">{album.artist}</h2>

        <Controls />

        <h2 className="text-center text-lg mt-4">
          {trackIndex! + 1}. {track.name}
        </h2>

        <Seeker />
      </div>

      <ol className="list-decimal list-inside space-y-2 mt-8">
        {tracks.map((t, index) => (
          <li key={index} className="">
            <div>{t.name}</div>
          </li>
        ))}
      </ol>
    </div>
  );
}

export default Player;
