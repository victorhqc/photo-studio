import { combineReducers, Reducer } from 'redux';
import { ActionType, getType } from 'typesafe-actions';
import { AuthenticatedUserState } from './types';
import * as actions from './actions';

export type AuthAction = ActionType<typeof actions>;

const initialAuthenticatedUserState: AuthenticatedUserState = null;

export const user: Reducer<AuthenticatedUserState, AuthAction> = (
  state = initialAuthenticatedUserState,
  action
) => {
  switch (action.type) {
    case getType(actions.authenticate):
      return action.payload;
    default:
      return null;
  }
};

const auth = combineReducers({ user });

export default { auth };
