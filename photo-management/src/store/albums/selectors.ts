import { createSelector } from 'reselect';
import { ApplicationState } from '../index';
import { AlbumWithPhotos } from './types';

export const selectAlbums = (state: ApplicationState) => {
  const albums = state.albums.list.data;

  if (!albums) {
    return [];
  }

  return albums;
};

export const selectAlbumById = createSelector(
  selectAlbums,
  (_state: ApplicationState, albumId: string) => albumId,
  (albums, albumId): AlbumWithPhotos | null => {
    if (!albums) return null;

    const album = albums.find(([album]) => album.id === albumId);

    return album || null;
  }
);

export const selectOpenedAlbum = (state: ApplicationState) => {
  return state.albums.openedAlbum.data;
};

export const selectOpenedAlbumOrFail = (state: ApplicationState) => {
  const album = selectOpenedAlbum(state);

  if (!album) {
    throw new Error('No opened album');
  }

  return album;
};

export const selectUploadStatus = (state: ApplicationState) => {
  return state.albums.openedAlbum.upload;
};

export const selectNeedsRebuild = (state: ApplicationState) =>
  state.albums.openedAlbum.needsRebuild;
