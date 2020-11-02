import { combineReducers, Reducer } from 'redux';
import { ActionType, getType } from 'typesafe-actions';
import { BookMeState } from './types';
import * as actions from './actions';

export type BookMeAction = ActionType<typeof actions>;

const initialInfo: BookMeState = {
  status: 'idle',
  data: null,
};

export const info: Reducer<BookMeState, BookMeAction> = (state = initialInfo, action) => {
  switch (action.type) {
    case getType(actions.fetchBookMeInfo.request):
      return {
        data: null,
        status: 'loading',
      };

    case getType(actions.fetchBookMeInfo.success):
      return {
        data: action.payload,
        status: 'done',
      };

    case getType(actions.fetchBookMeInfo.failure):
      return {
        error: action.payload,
        status: 'error',
      };

    case getType(actions.updateBookMeInfo.request):
      return {
        ...state,
        data: {
          ...(state.data || { id: '', email: '', userId: '' }),
          email: action.payload.email,
        },
        status: 'loading',
      };

    case getType(actions.updateBookMeInfo.success):
      return {
        data: action.payload,
        status: 'done',
      };

    case getType(actions.updateBookMeInfo.failure):
      return {
        data: {
          ...(state.data || { id: '', email: '', userId: '' }),
          email: action.payload.email,
        },
        error: action.payload.error,
        status: 'error',
      };

    default:
      return state;
  }
};

const bookMe = combineReducers({ info });

export default { bookMe };
