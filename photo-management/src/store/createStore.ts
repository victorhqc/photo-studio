import { createStore, applyMiddleware } from 'redux';
import createSagaMiddleware from 'redux-saga';
import { routerMiddleware } from 'connected-react-router';
import { History } from 'history';
import { composeWithDevTools } from 'redux-devtools-extension';
import { Reducer } from './index';
import { localStorageMiddleware, getStateFromLocalStorage } from './localStorageMiddleware';
import rootReducer from './rootReducer';
import rootSaga from './rootSaga';
import { ApiMaker } from '../api';

const sagaMiddleware = createSagaMiddleware({});
const composeEnhancers = composeWithDevTools({});

const buildStore = (history: History) => {
  const reducersToPersist: Reducer[] = ['auth'];
  const initialState = getStateFromLocalStorage(reducersToPersist);

  const store = createStore(
    rootReducer(history),
    initialState,
    composeEnhancers(
      applyMiddleware(
        routerMiddleware(history),
        sagaMiddleware,
        localStorageMiddleware(reducersToPersist)
      )
    )
  );

  sagaMiddleware.run(rootSaga);

  const api = ApiMaker.Api(store);
  window.__API__ = api;

  return store;
};

export type Store = ReturnType<typeof buildStore>;

export default buildStore;
