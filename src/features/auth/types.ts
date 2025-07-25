export interface SessionResponse {
  authenticated: boolean;
}

export interface JellyfinItemsResponse {
  items: JellyfinItem[];
  totalRecordCount: number;
  startIndex: number;
}

export interface JellyfinItem {
  id: string;
  name: string;
  albumArtist?: string;
}
