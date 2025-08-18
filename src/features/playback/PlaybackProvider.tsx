import { convertFileSrc } from "@tauri-apps/api/core";
import {
  PropsWithChildren,
  createContext,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";

type PlaybackContextType = {
  album: Album | null;
  setAlbum: (album: Album | null) => void;
  track: AlbumTrack | null;
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  togglePlayPause: () => void;
  handleSeek: (position: number) => void;
  handleNextTrack: () => void;
  hasNextTrack: boolean;
  handlePreviousTrack: () => void;
  hasPreviousTrack: boolean;
  tracks: AlbumTrack[];
  trackIndex: number | null;
  setTrackIndex: (index: number | null) => void;
};

const PlaybackContext = createContext<PlaybackContextType | undefined>(
  undefined
);

function setMediaSessionMedadata(
  title: string,
  artist: string,
  album: string,
  albumArtUrl?: string
) {
  if ("mediaSession" in navigator) {
    const albumArt = albumArtUrl ? convertFileSrc(albumArtUrl) : null;

    navigator.mediaSession.metadata = new MediaMetadata({
      title,
      artist,
      album,
      artwork: albumArt
        ? [
            {
              src: albumArt,
              sizes: "300x300",
            },
          ]
        : [],
    });
  }
}

export const PlaybackProvider = ({ children }: PropsWithChildren) => {
  const [album, setAlbum] = useState<Album | null>(null);
  const [track, setTrack] = useState<AlbumTrack | null>(null);
  const [trackIndex, setTrackIndex] = useState<number | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const [autoPlay, setAutoPlay] = useState(false);

  // selects first track when album is set
  useEffect(() => {
    if (album && album.tracks && album.tracks.length > 0) {
      setTrackIndex(0);
      setAutoPlay(true);
    } else {
      setTrackIndex(null);
    }
  }, [album]);

  // sets the track when trackIndex changes
  useEffect(() => {
    if (trackIndex !== null && album && album.tracks) {
      setTrack(album.tracks[trackIndex]);
    } else {
      setTrack(null);
    }
  }, [trackIndex, album]);

  // sets up the audio when the track changes
  useEffect(() => {
    if (!track) {
      return;
    }

    setCurrentTime(0);

    const src = convertFileSrc(track.playbackUrl);
    const audio = new Audio(src);
    audioRef.current = audio;

    const handlePlay = () => {
      setIsPlaying(true);
    };

    const handlePause = () => {
      setIsPlaying(false);
    };

    const handleTimeUpdate = () => {
      setCurrentTime(audio.currentTime);
    };

    const handleLoadedMetadata = () => setDuration(audio.duration);

    const handleEnded = () => {
      if (
        trackIndex !== null &&
        trackIndex < (album?.tracks?.length ?? 0) - 1
      ) {
        handleNextTrack();
      } else {
        setIsPlaying(false);
        setAutoPlay(false);
      }
    };

    audio.addEventListener("play", handlePlay);
    audio.addEventListener("pause", handlePause);
    audio.addEventListener("timeupdate", handleTimeUpdate);
    audio.addEventListener("loadedmetadata", handleLoadedMetadata);
    audio.addEventListener("ended", handleEnded);

    if (autoPlay) {
      audio.play();
    }

    return () => {
      audio.removeEventListener("play", handlePlay);
      audio.removeEventListener("pause", handlePause);
      audio.removeEventListener("timeupdate", handleTimeUpdate);
      audio.removeEventListener("loadedmetadata", handleLoadedMetadata);
      audio.removeEventListener("ended", handleEnded);
      audio.pause();
      audio.src = "";
    };
  }, [track]);

  const togglePlayPause = () => {
    const audio = audioRef.current;
    if (!audio) {
      return;
    }

    if (isPlaying) {
      audio.pause();
      setAutoPlay(false);
    } else {
      audio.play();
      setAutoPlay(true);
    }
  };

  const handleSeek = (position: number) => {
    const audio = audioRef.current;
    if (!audio) {
      return;
    }

    audio.currentTime = position;
  };

  const handleNextTrack = () => {
    if (!album || !album.tracks || album.tracks.length === 0) {
      return;
    }

    const nextIndex = (trackIndex ?? 0) + 1;
    if (nextIndex < album.tracks.length) {
      setTrackIndex(nextIndex);
    }
  };

  const hasNextTrack =
    trackIndex !== null && trackIndex < (album?.tracks?.length ?? 0) - 1;

  const handlePreviousTrack = () => {
    if (!album || !album.tracks || album.tracks.length === 0) {
      return;
    }

    const prevIndex = (trackIndex ?? 0) - 1;
    if (prevIndex >= 0) {
      setTrackIndex(prevIndex);
    }
  };

  const hasPreviousTrack = trackIndex !== null && trackIndex > 0;

  useEffect(() => {
    if (!track || !album) {
      return;
    }

    setMediaSessionMedadata(
      track.name,
      album.artist,
      album.name,
      album.imageUrl
    );
  }, [track, album]);

  useEffect(() => {
    if ("mediaSession" in navigator) {
      navigator.mediaSession.setActionHandler("play", togglePlayPause);
      navigator.mediaSession.setActionHandler("pause", togglePlayPause);
      navigator.mediaSession.setActionHandler("nexttrack", handleNextTrack);
      navigator.mediaSession.setActionHandler(
        "previoustrack",
        handlePreviousTrack
      );
    }
  }, [togglePlayPause, handleNextTrack, handlePreviousTrack]);

  return (
    <PlaybackContext.Provider
      value={{
        album,
        setAlbum,
        track,
        isPlaying,
        currentTime,
        duration,
        togglePlayPause,
        handleSeek,
        handleNextTrack,
        hasNextTrack,
        handlePreviousTrack,
        hasPreviousTrack,
        tracks: album?.tracks || [],
        trackIndex,
        setTrackIndex,
      }}
    >
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
