import { combineReducers } from 'redux';
import { History } from 'history';
import { connectRouter } from 'connected-react-router';

import authReducers, { AuthAction } from './auth/reducer';
import albumReducers, { AlbumAction } from './albums/reducer';

const rootReducer = (history: History) =>
  combineReducers({
    router: connectRouter(history),
    ...authReducers,
    ...albumReducers,
  });

export default rootReducer;

export type ApplicationState = ReturnType<ReturnType<typeof rootReducer>>;
export type ApplicationAction = AuthAction | AlbumAction;
export type Reducer = keyof ApplicationState;
