import { call, put, select } from 'typed-redux-saga';
import { takeEvery, select as _select } from 'redux-saga/effects';
import { ActionMatchingPattern as ActionType } from '@redux-saga/types';
import { fetchAllAlbums, fetchAlbumPhotos, addPhoto, openAlbum, deletePhoto } from './actions';
import { selectOpenedAlbumOrFail, selectAlbumById } from './selectors';
import { getApi } from '../../api';

export default function* albumsSaga() {
  yield takeEvery(fetchAllAlbums.request, handlefetchAllAlbums);
  yield takeEvery(fetchAlbumPhotos.request, handleFetchAlbumPhotos);
  yield takeEvery(addPhoto.request, handleAddPhoto);
  yield takeEvery(deletePhoto.request, handleDeletePhoto);
}

function* handlefetchAllAlbums(action: ActionType<typeof fetchAllAlbums.request>) {
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
    // @ts-ignore
    const album = yield* select(selectAlbumById, action.payload);
    if (!album) {
      throw new Error('Album does not exist');
    }
    yield put(openAlbum(album[0]));

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
    });
    yield put(addPhoto.success(photo));
  } catch (e) {
    yield put(addPhoto.failure(e));
  }
}

function* handleDeletePhoto(action: ActionType<typeof deletePhoto.request>) {
  try {
    const api = getApi();

    yield* call(api.deletePhoto, { id: action.payload });

    yield put(deletePhoto.success());
  } catch (e) {
    yield put(deletePhoto.failure(e));
  }
}
