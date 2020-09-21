import { ApplicationState } from '../index';

export const selectAuthenticatedUser = (store: ApplicationState) => store.auth.user;
