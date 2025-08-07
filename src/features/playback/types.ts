interface Album {
  name: string;
  artist: string;
  tracks: AlbumTrack[];
}

interface AlbumTrack {
  name: string;
  playbackUrl: string;
}
