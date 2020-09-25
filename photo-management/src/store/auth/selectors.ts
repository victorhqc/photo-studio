import { ApplicationState } from '../index';

export const selectAuthenticatedUser = (store: ApplicationState) => {
  const user = store.auth.user.data;

  if (!user) {
    throw new Error('Not authenticated');
  }

  return user;
};

export const selectMaybeAuthenticatedUser = (store: ApplicationState) => {
  try {
    return selectAuthenticatedUser(store);
  } catch (e) {
    return null;
  }
};

export const selectToken = (store: ApplicationState) => store.auth.token;
