import { createAsyncAction } from 'typesafe-actions';
import { AlbumWithPhotos, Photo } from './types';

export const fetchAlbums = createAsyncAction(
  'albums/fetch',
  'albums/fetch_success',
  'albums/fetch_error',
  'albums/fetch_cancel'
)<void, AlbumWithPhotos[], Error, void>();

export const fetchAlbumPhotos = createAsyncAction(
  'albums/fetch_photos',
  'albums/fetch_photos_success',
  'albums/fetch_photos_error',
  'albums/fetch_photos_cancel'
)<void, Photo[], Error, void>();

export const addPhoto = createAsyncAction(
  'albums/add_photo',
  'albums/add_photo_success',
  'albums/add_photo_error',
  'albums/add_photo_cancel'
)<{ img: File; color: string; name: string; description: string | null }, Photo, Error, void>();
