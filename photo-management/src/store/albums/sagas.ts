import { call, put } from 'typed-redux-saga';
import { takeEvery } from 'redux-saga/effects';
import { ActionMatchingPattern as ActionType } from '@redux-saga/types';
import { fetchAlbums, addPhoto } from './actions';
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

    const response = yield* call(api.uploadPhoto, action.payload.img);
    console.log('response', response);
  } catch (e) {
    yield put(addPhoto.failure(e));
  }
}
