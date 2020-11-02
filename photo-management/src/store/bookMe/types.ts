import { AsyncData } from '../../utils/types';

export type BookMeState = AsyncData<BookMe>;

export type BookMe = {
  id: string;
  userId: string;
  email: string;
};
