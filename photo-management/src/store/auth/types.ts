import { AsyncData } from '../../utils/types';

export type AuthenticatedUser = {
  id: string;
  email: string;
  picture: string;
};

export type AuthenticatedUserState = AsyncData<AuthenticatedUser>;
