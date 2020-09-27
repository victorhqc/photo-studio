import { createAsyncAction } from 'typesafe-actions';
import { AlbumWithPhotos, Photo } from './types';

export const fetchAllAlbums = createAsyncAction(
  'albums/fetch_all',
  'albums/fetch_all_success',
  'albums/fetch_all_error',
  'albums/fetch_all_cancel'
)<void, AlbumWithPhotos[], Error, void>();

export const fetchAlbum = createAsyncAction(
  'albums/fetch_album',
  'albums/fetch_album_success',
  'albums/fetch_album_error',
  'albums/fetch_album_cancel'
)<string, AlbumWithPhotos, Error, void>();

export const addPhoto = createAsyncAction(
  'albums/add_photo',
  'albums/add_photo_success',
  'albums/add_photo_error',
  'albums/add_photo_cancel'
)<{ img: File; color: string; name: string; description: string | null }, Photo, Error, void>();
