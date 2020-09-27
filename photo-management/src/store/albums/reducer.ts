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
    case getType(actions.fetchAllAlbums.request):
      return { status: 'loading', data: null };
    case getType(actions.fetchAllAlbums.success):
      return { status: 'done', data: action.payload };
    case getType(actions.fetchAllAlbums.failure):
      return { status: 'error', error: action.payload };
    case getType(actions.fetchAllAlbums.cancel):
      return { status: 'idle', data: null };
    default:
      return state;
  }
};

const initialOpenedAlbum: AlbumOpenedState = {
  status: 'idle',
  data: null,
  upload: 'idle',
};

export const openedAlbum: Reducer<AlbumOpenedState, AlbumAction> = (
  state = initialOpenedAlbum,
  action
) => {
  switch (action.type) {
    case getType(actions.openAlbum):
      return {
        ...state,
        data: [action.payload, []],
      };

    case getType(actions.fetchAlbumPhotos.request):
      return {
        ...state,
        status: 'loading',
      };

    case getType(actions.fetchAlbumPhotos.success):
      if (!state.data) {
        console.warn('Received photos before opening album');
        return state;
      }

      return {
        ...state,
        status: 'done',
        data: [state.data[0], action.payload],
      };

    case getType(actions.fetchAlbumPhotos.failure):
      return {
        ...state,
        status: 'error',
        error: action.payload,
      };

    case getType(actions.addPhoto.request):
      return {
        ...state,
        upload: 'loading',
      };
    case getType(actions.addPhoto.success):
      if (!state.data)
        return {
          ...state,
          upload: 'done',
        };

      return {
        ...state,
        data: [state.data[0], [...state.data[1], action.payload]],
        upload: 'done',
      };
    case getType(actions.addPhoto.failure):
      return {
        ...state,
        upload: 'error',
      };
    default:
      return state;
  }
};

const albums = combineReducers({ list, openedAlbum });

export default { albums };
