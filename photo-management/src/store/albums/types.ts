import { AsyncData, AsyncStatus } from '../../utils/types';

export type AlbumsState = AsyncData<AlbumWithPhotos[]>;

export type AlbumOpenedState = AsyncData<AlbumWithPhotos> & {
  upload: AsyncStatus;
  needsRebuild: boolean;
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
  title: string | null;
  description: string | null;
  width: number;
  height: number;
  isFavorite: boolean;
  createdAt: number;
  updatedAt: number;
  deleted: boolean;
};
