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
  needsRebuild: false,
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
          needsRebuild: true,
        };

      return {
        ...state,
        data: [state.data[0], [...state.data[1], action.payload]],
        needsRebuild: true,
        upload: 'done',
      };
    case getType(actions.addPhoto.failure):
      return {
        ...state,
        upload: 'error',
      };

    case getType(actions.updatePhoto.success): {
      if (!state.data) return state;

      const index = state.data[1].findIndex((p) => p.id === action.payload.id);

      return {
        ...state,
        data: [
          state.data[0],
          [
            ...state.data[1].slice(0, index),
            action.payload,
            ...state.data[1].slice(index + 1, state.data[1].length),
          ],
        ],
        needsRebuild: true,
      };
    }

    case getType(actions.deletePhoto.success): {
      if (!state.data) {
        return state;
      }

      const index = state.data[1].findIndex((photo) => photo.id === action.payload);

      const photos = [
        ...state.data[1].slice(0, index),
        ...state.data[1].slice(index + 1, state.data[1].length),
      ];

      return {
        ...state,
        data: [state.data[0], photos],
        needsRebuild: true,
      };
    }

    case getType(actions.buildApplication):
      return {
        ...state,
        needsRebuild: false,
      };
    default:
      return state;
  }
};

const albums = combineReducers({ list, openedAlbum });

export default { albums };
