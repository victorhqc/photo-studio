import { all, spawn } from 'redux-saga/effects';
import authSaga from './auth/sagas';
import albumsSaga from './albums/sagas';
import bookMeSaga from './bookMe/sagas';

export default function* rootSata() {
  yield all([spawn(authSaga), spawn(albumsSaga), spawn(bookMeSaga)]);
}
