import { ApplicationState } from '../index';

export const selectAlbums = (state: ApplicationState) => {
  const albums = state.albums.list.data;

  if (!albums) {
    return [];
  }

  return albums;
};
