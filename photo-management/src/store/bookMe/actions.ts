import { createAsyncAction } from 'typesafe-actions';
import { BookMe } from './types';

export const fetchBookMeInfo = createAsyncAction(
  'book_me/fetch',
  'book_me/fetch_success',
  'book_me/fetch_error',
  'book_me/fetch_cancel'
)<void, BookMe, Error, void>();

export const updateBookMeInfo = createAsyncAction(
  'book_me/update',
  'book_me/update_success',
  'book_me/update_error',
  'book_me/update_cancel'
)<{ email: string }, BookMe, { error: Error; email: string }, void>();
