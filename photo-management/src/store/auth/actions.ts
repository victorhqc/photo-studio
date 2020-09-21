import { createAsyncAction } from 'typesafe-actions';
import { AuthenticatedUser } from './types';

export const authenticate = createAsyncAction(
  'auth/authenticate',
  'auth/authenticate_success',
  'auth/authenticate/error',
  'auth/authenticate/cancel'
)<string, AuthenticatedUser, Error, void>();
