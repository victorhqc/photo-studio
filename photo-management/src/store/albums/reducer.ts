import { combineReducers, Reducer } from 'redux';
import { ActionType, getType } from 'typesafe-actions';
import { AlbumsState, AlbumOpenedState } from './types';
import * as actions from './actions';

export type AlbumAction = ActionType<typeof actions>;

const initialAlbums: AlbumsState = {
  status: 'idle',
  data: null,
};

export const list: Reducer<AlbumsState, AlbumAction> = (state = initialAlbums, action) => {
  switch (action.type) {
    case getType(actions.fetchAlbums.request):
      return { status: 'loading', data: null };
    case getType(actions.fetchAlbums.success):
      return { status: 'done', data: action.payload };
    case getType(actions.fetchAlbums.failure):
      return { status: 'error', error: action.payload };
    case getType(actions.fetchAlbums.cancel):
      return { status: 'idle', data: null };
    default:
      return state;
  }
};

const initialOpenedAlbum: AlbumOpenedState = {
  status: 'idle',
  data: null,
};

export const openedAlbum: Reducer<AlbumOpenedState, AlbumAction> = (
  state = initialOpenedAlbum,
  action
) => {
  switch (action.type) {
    case getType(actions.addPhoto.success):
      if (!state.data) return state;

      return {
        ...state,
        data: [state.data[0], [...state.data[1], action.payload]],
      };
    default:
      return state;
  }
};

const albums = combineReducers({ list, openedAlbum });

export default { albums };
