import { Store } from 'redux';
import { selectToken, logout } from '../store/auth';

export const ApiMaker = {
  Api(store: Store) {
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

    const res = await fetch(`${ApiMaker.forgeUrl()}${path}`, {
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

  apiInstance: (store: Store) => {
    return {
      getMe: function (): Promise<AuthenticatedUser | null> {
        return ApiMaker.get(store, '/api/me');
      },
    };
  },
};

type Api = ReturnType<typeof ApiMaker['Api']>;

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

export type User = {
  id: string;
  email: string;
};
