export interface SessionResponse {
  authenticated: boolean;
}

export interface AlbumSearchResponse {
  items: AlbumSearchResponseItem[];
  totalRecordCount: number;
  startIndex: number;
}

export interface AlbumSearchResponseItem {
  id: string;
  name: string;
  albumArtist?: string;
  downloaded: boolean;
}
