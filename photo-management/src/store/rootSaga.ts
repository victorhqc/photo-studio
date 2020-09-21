import { all, spawn } from 'redux-saga/effects';
import authSaga from './auth/sagas';

export default function* rootSata() {
  yield all([spawn(authSaga)]);
}
