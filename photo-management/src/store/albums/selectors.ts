import { ApplicationState } from '../index';

export const selectAlbums = (state: ApplicationState) => {
  const albums = state.albums.list.data;

  if (!albums) {
    return [];
  }

  return albums;
};

export const selectOpenedAlbum = (state: ApplicationState) => {
  const albums = state.albums.list.data;

  if (!albums) {
    throw new Error('No opened album');
  }

  return albums[0];
};

export const selectUploadStatus = (state: ApplicationState) => {
  return state.albums.openedAlbum.upload;
};
