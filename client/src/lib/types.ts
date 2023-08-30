export interface Song {
  id: string;
  name: string;
  artist: string;
  album: string;
  duration: number;
  album_url?: string;
}

export interface WSMessage {
  channel: string;
  message: string;
  artist?: string;
  song?: string;
}
