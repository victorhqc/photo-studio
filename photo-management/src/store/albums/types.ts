import { AsyncData } from '../../utils/types';

export type AlbumsState = AsyncData<AlbumWithPhotos[]>;

export type AlbumWithPhotos = [Album, Photo[]];

export type Album = {
  id: string;
  userId: string;
  name: string;
  description: string | null;
  createdAt: number;
  updatedAt: number;
  deleted: boolean;
};

export type Photo = {
  id: string;
  albumId: string;
  userId: string;
  indexInAlbum: number;
  src: string;
  main_color: string;
  description: string | null;
  created_at: number;
  updated_at: number;
  deleted: boolean;
};
