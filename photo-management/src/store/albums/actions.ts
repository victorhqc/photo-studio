import { createAsyncAction, createAction } from 'typesafe-actions';
import { AlbumWithPhotos, Album, Photo } from './types';

export const fetchAllAlbums = createAsyncAction(
  'albums/fetch_all',
  'albums/fetch_all_success',
  'albums/fetch_all_error',
  'albums/fetch_all_cancel'
)<void, AlbumWithPhotos[], Error, void>();

export const openAlbum = createAction('albums/open')<Album>();

export const fetchAlbumPhotos = createAsyncAction(
  'albums/fetch_photos',
  // Looks like tuples don't work correctly
  'albums/fetch_photos_success',
  'albums/fetch_photos_error',
  'albums/fetch_photos_cancel'
)<string, Photo[], Error, void>();

export const addPhoto = createAsyncAction(
  'albums/add_photo',
  'albums/add_photo_success',
  'albums/add_photo_error',
  'albums/add_photo_cancel'
)<{ img: File; color: string } & Description & Dimensions, Photo, Error, void>();

export const updatePhoto = createAsyncAction(
  'albums/update_photo',
  'albums/update_photo_success',
  'albums/update_photo_error',
  'albums/update_photo_cancel'
)<{ id: string; isFavorite: boolean } & Description, Photo, Error, void>();

export const deletePhoto = createAsyncAction(
  'albums/delete_photo',
  'albums/delete_photo_success',
  'albums/delete_photo_error',
  'albums/delete_photo_abort'
)<string, string, Error, void>();

export const buildApplication = createAction('albums/build_app')<void>();

type Description = { title: string | null; description: string | null };
type Dimensions = { width: number; height: number };
