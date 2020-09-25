import { call, put } from 'typed-redux-saga';
import { takeEvery } from 'redux-saga/effects';
import { ActionMatchingPattern as ActionType } from '@redux-saga/types';
import { fetchAlbums } from './actions';
import { getApi } from '../../api';

export default function* albumsSaga() {
  yield takeEvery(fetchAlbums.request, handleFetchAlbums);
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
