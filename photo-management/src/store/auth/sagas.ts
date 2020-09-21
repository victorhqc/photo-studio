import { call, takeEvery, put } from 'typed-redux-saga';
import { getType } from 'typesafe-actions';
import { ActionMatchingPattern as ActionType } from '@redux-saga/types';
import { push } from 'connected-react-router';
import { authenticate } from './actions';
import { getMe } from '../../auth';

export default function* authSaga() {
  yield takeEvery(getType(authenticate.request), handleAuthentication);
}

function* handleAuthentication(action: ActionType<typeof authenticate.request>) {
  const me = yield* call(getMe, action.payload);

  yield put(
    authenticate.success({
      id: me.user.id,
      email: me.user.email,
      token: action.payload,
    })
  );

  yield put(push('/home'));
}
