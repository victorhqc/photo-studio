import { AsyncData, AsyncStatus } from '../../utils/types';

export type AlbumsState = AsyncData<AlbumWithPhotos[]>;

export type AlbumOpenedState = AsyncData<AlbumWithPhotos> & {
  upload: AsyncStatus;
};

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
  mainColor: string;
  createdAt: number;
  updatedAt: number;
  deleted: boolean;
};
