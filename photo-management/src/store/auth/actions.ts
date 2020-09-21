import { createAction } from 'typesafe-actions';
import { AuthenticatedUser } from './types';

export const authenticate = createAction('auth/authenticate')<AuthenticatedUser>();
