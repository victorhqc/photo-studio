import { ApplicationState } from '../index';

export const selectAuthenticatedUser = (store: ApplicationState) => store.auth.user.data;
export const selectToken = (store: ApplicationState) => store.auth.token;
