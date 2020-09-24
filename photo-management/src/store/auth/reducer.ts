import { combineReducers, Reducer } from 'redux';
import { ActionType, getType } from 'typesafe-actions';
import { AuthenticatedUserState } from './types';
import * as actions from './actions';

export type AuthAction = ActionType<typeof actions>;

const initialAuthenticatedUserState: AuthenticatedUserState = {
  status: 'idle',
  data: null,
};

export const user: Reducer<AuthenticatedUserState, AuthAction> = (
  state = initialAuthenticatedUserState,
  action
) => {
  switch (action.type) {
    case getType(actions.authenticate.request):
      return { status: 'loading', data: null };
    case getType(actions.authenticate.success):
      return { status: 'done', data: action.payload };
    case getType(actions.authenticate.failure):
      return { status: 'error', error: action.payload };
    case getType(actions.authenticate.cancel):
      return { status: 'idle', data: null };
    case getType(actions.logout):
      return { status: 'idle', data: null };
    default:
      return state;
  }
};

const token: Reducer<string | null, AuthAction> = (state = null, action) => {
  switch (action.type) {
    case getType(actions.setToken):
      return action.payload;
    default:
      return state;
  }
};

const auth = combineReducers({ user, token });

export default { auth };
