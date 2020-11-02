import { ApplicationState } from '../index';

export const selectBookMeInfo = (state: ApplicationState) => state.bookMe.info;

export const selectBookMeEmail = (state: ApplicationState) => state.bookMe.info.data?.email;
