import { AsyncData } from '../../utils/types';

export type AuthenticatedUser = {
  id: string;
  email: string;
};

export type AuthenticatedUserState = AsyncData<AuthenticatedUser>;
