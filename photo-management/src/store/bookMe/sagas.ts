import { call, put } from 'typed-redux-saga';
import { takeEvery } from 'redux-saga/effects';
import { ActionMatchingPattern as ActionType } from '@redux-saga/types';
import { fetchBookMeInfo, updateBookMeInfo } from './actions';
import { getApi } from '../../api';

export default function* bookMeSaga() {
  yield takeEvery(fetchBookMeInfo.request, handleFetchBookMeInfo);
  yield takeEvery(updateBookMeInfo.request, handleUpdateBookMeInfo);
}

function* handleFetchBookMeInfo() {
  try {
    const api = getApi();
    const { info } = yield* call(api.getBookMeInfo);

    yield put(fetchBookMeInfo.success(info));
  } catch (e) {
    console.log(e);
    yield put(fetchBookMeInfo.failure(e));
  }
}

function* handleUpdateBookMeInfo(action: ActionType<typeof updateBookMeInfo.request>) {
  try {
    const api = getApi();
    const { info } = yield* call(api.updateBookMe, { email: action.payload.email });

    yield put(updateBookMeInfo.success(info));
  } catch (e) {
    console.log(e);
    yield put(updateBookMeInfo.failure({ error: e, email: action.payload.email }));
  }
}
