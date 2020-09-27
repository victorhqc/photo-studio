import { Store } from 'redux';
import merge from 'lodash/merge';
import { selectToken, logout, AuthenticatedUser as User } from '../store/auth';
import { AlbumWithPhotos, Photo } from '../store/albums';

export const ApiFactory = {
  Build(store: Store) {
    return this.apiInstance(store);
  },

  forgeUrl(): string {
    return process.env.REACT_APP_API_URL || 'REACT_APP_API_URL is not configured';
  },

  request: async <T>(store: Store, path: string, options: RequestOptions): Promise<T | null> => {
    const token = selectToken(store.getState());

    if (!token) {
      return null;
    }

    const res = await fetch(
      `${ApiFactory.forgeUrl()}${path}`,
      merge(options, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      })
    );

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

  get: async <T>(store: Store, path: string): Promise<T | null> => {
    return ApiFactory.request<T>(store, path, {
      method: 'GET',
    });
  },

  getOrFail: async <T>(store: Store, path: string): Promise<T> => {
    const result = await ApiFactory.get<T>(store, path);

    if (!result) {
      throw new Error('Result is empty');
    }

    return result;
  },

  postMultipart: async <T>(store: Store, path: string, body: FormData): Promise<T> => {
    const result = await ApiFactory.request<T>(store, path, {
      method: 'POST',
      body,
    });

    if (!result) {
      throw new Error('Result is empty');
    }

    return result;
  },

  post: async <T>(store: Store, path: string, data: object): Promise<T> => {
    const result = await ApiFactory.request<T>(store, path, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    });

    if (!result) {
      throw new Error('Could not post data');
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
      getAlbumPhotos: function getAlbumPhotos(albumId: string): Promise<{ list: Photo[] }> {
        return ApiFactory.getOrFail(store, `/api/album/${albumId}/photos`);
      },
      uploadPhoto: function uploadPhoto(file: File): Promise<{ photoUrl: string }> {
        const body = new FormData();
        body.append('photo', file);

        return ApiFactory.postMultipart(store, '/api/photo/upload', body);
      },
      newPhoto: function newPhoto({
        albumId,
        src,
        mainColor,
        name,
        description,
      }: AddPhotoArgs): Promise<{ photo: Photo }> {
        return ApiFactory.post(store, `/api/album/${albumId}/photo`, {
          indexInAlbum: 0,
          name,
          description,
          src,
          mainColor,
        });
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

type RequestOptions = {
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
  body?: Blob | BufferSource | FormData | URLSearchParams | ReadableStream<Uint8Array> | string;
  headers?: Record<string, string>;
};

type AddPhotoArgs = {
  albumId: string;
  src: string;
  mainColor: string;
  name: string;
  description: string | null;
};
