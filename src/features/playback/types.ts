interface Album {
  name: string;
  artist: string;
  tracks: AlbumTrack[];
  imageUrl?: string;
}

interface AlbumTrack {
  name: string;
  playbackUrl: string;
}
