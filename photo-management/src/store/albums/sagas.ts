import { call, put, select } from 'typed-redux-saga';
import { takeEvery } from 'redux-saga/effects';
import { ActionMatchingPattern as ActionType } from '@redux-saga/types';
import {
  fetchAllAlbums,
  fetchAlbumPhotos,
  addPhoto,
  deletePhoto,
  updatePhoto,
  buildApplication,
} from './actions';
import { selectOpenedAlbumOrFail } from './selectors';
import { getApi } from '../../api';
import { rebuildApplication } from '../../utils/netlify';

export default function* albumsSaga() {
  yield takeEvery(fetchAllAlbums.request, handlefetchAllAlbums);
  yield takeEvery(fetchAlbumPhotos.request, handleFetchAlbumPhotos);
  yield takeEvery(addPhoto.request, handleAddPhoto);
  yield takeEvery(deletePhoto.request, handleDeletePhoto);
  yield takeEvery(updatePhoto.request, handleUpdatePhoto);
  yield takeEvery(buildApplication, handleBuildApplication);
}

function* handlefetchAllAlbums() {
  try {
    const api = getApi();

    const { list: albums } = yield* call(api.getAlbums);

    yield put(fetchAllAlbums.success(albums));
  } catch (e) {
    yield put(fetchAllAlbums.failure(e));
  }
}

function* handleFetchAlbumPhotos(action: ActionType<typeof fetchAlbumPhotos.request>) {
  try {
    const api = getApi();
    const { list: photos } = yield* call(api.getAlbumPhotos, action.payload);

    yield put(fetchAlbumPhotos.success(photos));
  } catch (e) {
    console.log(e);
    yield put(fetchAlbumPhotos.failure(e));
  }
}

function* handleAddPhoto(action: ActionType<typeof addPhoto.request>) {
  try {
    const api = getApi();
    const [album] = yield* select(selectOpenedAlbumOrFail);

    const response = yield* call(api.uploadPhoto, action.payload.img);

    const { photo } = yield* call(api.newPhoto, {
      s3Id: response.s3Id,
      albumId: album.id,
      src: response.photoUrl,
      mainColor: action.payload.color,
      title: action.payload.title,
      description: action.payload.description,
      width: action.payload.width,
      height: action.payload.height,
    });
    yield put(addPhoto.success(photo));
  } catch (e) {
    yield put(addPhoto.failure(e));
  }
}

function* handleUpdatePhoto(action: ActionType<typeof updatePhoto.request>) {
  try {
    const api = getApi();

    const { photo } = yield* call(api.updatePhoto, {
      id: action.payload.id,
      isFavorite: action.payload.isFavorite,
      indexInAlbum: 0,
      title: action.payload.title,
      description: action.payload.description,
    });

    yield put(updatePhoto.success(photo));
  } catch (e) {
    yield put(updatePhoto.failure(e));
  }
}

function* handleDeletePhoto(action: ActionType<typeof deletePhoto.request>) {
  try {
    const api = getApi();

    yield* call(api.deletePhoto, { id: action.payload });

    yield put(deletePhoto.success(action.payload));
  } catch (e) {
    yield put(deletePhoto.failure(e));
  }
}

function* handleBuildApplication() {
  try {
    yield call(rebuildApplication);
  } catch (e) {
    console.warn('Could not build application', e);
  }
}
