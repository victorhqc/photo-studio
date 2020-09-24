import { createAsyncAction, createAction } from 'typesafe-actions';
import { AuthenticatedUser } from './types';

export const setToken = createAction('auth/token')<string>();

export const authenticate = createAsyncAction(
  'auth/authenticate',
  'auth/authenticate_success',
  'auth/authenticate/error',
  'auth/authenticate/cancel'
)<string, AuthenticatedUser, Error, void>();

export const logout = createAction('auth/logout')<void>();

export const checkCredentials = createAction('auth/check-credentials')<void>();
