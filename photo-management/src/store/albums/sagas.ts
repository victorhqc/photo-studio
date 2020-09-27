import { call, put, select } from 'typed-redux-saga';
import { takeEvery } from 'redux-saga/effects';
import { ActionMatchingPattern as ActionType } from '@redux-saga/types';
import { fetchAlbums, addPhoto } from './actions';
import { selectOpenedAlbum } from './selectors';
import { getApi } from '../../api';

export default function* albumsSaga() {
  yield takeEvery(fetchAlbums.request, handleFetchAlbums);
  yield takeEvery(addPhoto.request, handleAddPhoto);
}

function* handleFetchAlbums(action: ActionType<typeof fetchAlbums.request>) {
  try {
    const api = getApi();

    const { list: albums } = yield* call(api.getAlbums);

    yield put(fetchAlbums.success(albums));
  } catch (e) {
    yield put(fetchAlbums.failure(e));
  }
}

function* handleAddPhoto(action: ActionType<typeof addPhoto.request>) {
  try {
    const api = getApi();
    const [album] = yield* select(selectOpenedAlbum);

    const response = yield* call(api.uploadPhoto, action.payload.img);

    const { photo } = yield* call(api.newPhoto, {
      albumId: album.id,
      src: response.photoUrl,
      mainColor: action.payload.color,
      name: action.payload.name,
      description: action.payload.description,
    });
    yield put(addPhoto.success(photo));
  } catch (e) {
    yield put(addPhoto.failure(e));
  }
}
