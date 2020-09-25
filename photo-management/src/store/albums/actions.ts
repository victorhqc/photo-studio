import { createAsyncAction } from 'typesafe-actions';
import { Album, Photo } from './types';

export const fetchAlbums = createAsyncAction(
  'albums/fetch',
  'albums/fetch_success',
  'albums/fetch_error',
  'albums/fetch_cancel'
)<void, Album[], Error, void>();

export const fetchAlbumPhotos = createAsyncAction(
  'albums/fetch_photos',
  'albums/fetch_photos_success',
  'albums/fetch_photos_error',
  'albums/fetch_photos_cancel'
)<void, Photo[], Error, void>();
