import { getApiUrl } from '../utils/env';

export async function getMe(token: string): Promise<AuthenticatedUser> {
  const res = await fetch(`${getApiUrl()}/api/me`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  if (!res.ok) {
    throw new Error(res.statusText);
  }

  if (res.status === 401) {
    throw new Error('Unauthorized');
  }

  if (res.status >= 500) {
    throw new Error(`Something went wrong: ${res.json()}`);
  }

  return res.json();
}

export type AuthenticatedUser = {
  user: User;
  token: string;
};

export type User = {
  id: string;
  email: string;
};
