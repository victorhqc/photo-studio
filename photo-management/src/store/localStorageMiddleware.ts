import { Middleware, MiddlewareAPI, Dispatch } from 'redux';
import isEqual from 'lodash/isEqual';
import { ApplicationState, Reducer } from './index';

export function localStorageMiddleware(reducers: Reducer[]) {
  let prevState = ({} as unknown) as ApplicationState;

  const localStorageReducerMiddleware: Middleware = (store: MiddlewareAPI) => (next: Dispatch) => (
    action
  ) => {
    const returnValue = next(action);

    const state: ApplicationState = store.getState();
    reducers.forEach((reducer) => {
      if (didStateChange(state, prevState, reducer)) {
        persistState(reducer, state);
      }
    });

    prevState = state;

    return returnValue;
  };

  return localStorageReducerMiddleware;
}

export function didStateChange(
  state: ApplicationState,
  prevState: ApplicationState,
  reducer: Reducer
) {
  const reducerState = state[reducer];
  const prevReducerState = prevState[reducer];

  return !isEqual(reducerState, prevReducerState);
}

function persistState(reducer: Reducer, state: ApplicationState) {
  if (typeof reducer !== 'string') {
    console.warn("Can't persist reducer", reducer);
    return;
  }

  localStorage.setItem(reducer, JSON.stringify(state[reducer]));
}

export function getStateFromLocalStorage(reducers: Reducer[]) {
  const state = ({} as unknown) as ApplicationState;

  for (const reducer of reducers) {
    if (typeof reducer !== 'string') continue;

    const strPersisted = localStorage.getItem(reducer);
    if (!strPersisted) continue;

    const persisted = JSON.parse(strPersisted);

    state[reducer] = persisted;
  }

  return state;
}
