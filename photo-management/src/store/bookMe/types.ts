import { AsyncData } from '../../utils/types';

export type BookMeState = AsyncData<BookMe>;

export type BookMe = {
  id: String;
  userId: String;
  email: String;
};
