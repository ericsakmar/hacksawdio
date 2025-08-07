import {
  PropsWithChildren,
  createContext,
  useContext,
  useEffect,
  useState,
} from "react";

type PlaybackContextType = {
  album: Album | null;
  setAlbum: (album: Album | null) => void;
  track: AlbumTrack | null;
};

const PlaybackContext = createContext<PlaybackContextType | undefined>(
  undefined
);

export const PlaybackProvider = ({ children }: PropsWithChildren) => {
  const [album, setAlbum] = useState<Album | null>(null);
  const [track, setTrack] = useState<AlbumTrack | null>(null);

  // selects first track when album is set
  useEffect(() => {
    if (album && album.tracks && album.tracks.length > 0) {
      setTrack(album.tracks[0]);
    } else {
      setTrack(null);
    }
  }, [album]);

  return (
    <PlaybackContext.Provider value={{ album, setAlbum, track }}>
      {children}
    </PlaybackContext.Provider>
  );
};

export const usePlayback = () => {
  const context = useContext(PlaybackContext);
  if (context === undefined) {
    throw new Error("usePlayback must be used within a PlaybackProvider");
  }
  return context;
};

