import { call, put } from 'typed-redux-saga';
import { takeEvery } from 'redux-saga/effects';
import { ActionMatchingPattern as ActionType } from '@redux-saga/types';
import { push } from 'connected-react-router';
import { setToken, authenticate, checkCredentials, logout } from './actions';
import { getApi } from '../../api';

export default function* authSaga() {
  yield takeEvery(authenticate.request, handleAuthentication);
  yield takeEvery(checkCredentials, handleCheckCredentials);
}

function* handleAuthentication(action: ActionType<typeof authenticate.request>) {
  const api = getApi();

  yield* put(setToken(action.payload));

  const me = yield* call(api.getMe);

  if (!me) {
    throw new Error('Could not authenticate');
  }

  yield put(
    authenticate.success({
      id: me.user.id,
      email: me.user.email,
    })
  );

  yield put(push('/home'));
}

function* handleCheckCredentials() {
  const api = getApi();

  try {
    yield* call(api.getMe);
  } catch (e) {
    console.warn(e);
    yield put(logout());
  }
}
