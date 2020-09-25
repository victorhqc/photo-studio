import { Store } from 'redux';
import { selectToken, logout, AuthenticatedUser as User } from '../store/auth';
import { AlbumWithPhotos } from '../store/albums';

export const ApiFactory = {
  Build(store: Store) {
    return this.apiInstance(store);
  },

  forgeUrl(): string {
    return process.env.REACT_APP_API_URL || 'REACT_APP_API_URL is not configured';
  },

  get: async <T>(store: Store, path: string): Promise<T | null> => {
    const token = selectToken(store.getState());

    if (!token) {
      return null;
    }

    const res = await fetch(`${ApiFactory.forgeUrl()}${path}`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });

    if (!res.ok) {
      throw new Error(res.statusText);
    }

    if (res.status === 401) {
      store.dispatch(logout());
      throw new Error('Unauthorized');
    }

    if (res.status >= 500) {
      throw new Error(`Something went wrong: ${res.json()}`);
    }

    return res.json();
  },

  getOrFail: async <T>(store: Store, path: string): Promise<T> => {
    const result = await ApiFactory.get<T>(store, path);

    if (!result) {
      throw new Error('Result is empty');
    }

    return result;
  },

  apiInstance: (store: Store) => {
    return {
      getMe: function getMe(): Promise<AuthenticatedUser | null> {
        return ApiFactory.get(store, '/api/me');
      },
      getAlbums: function getAlbums(): Promise<{ list: AlbumWithPhotos[] }> {
        return ApiFactory.getOrFail(store, '/api/albums');
      },
    };
  },
};

type Api = ReturnType<typeof ApiFactory['Build']>;

export function getApi(): Api {
  const api = window.__API__;

  if (!api) {
    throw new Error('Api is not ready');
  }

  return api;
}

export type AuthenticatedUser = {
  user: User;
  token: string;
};
