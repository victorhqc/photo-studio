import { AsyncData } from '../../utils/types';

export type AuthenticatedUser = {
  id: string;
  email: string;
  token: string;
};

export type AuthenticatedUserState = AsyncData<AuthenticatedUser>;
