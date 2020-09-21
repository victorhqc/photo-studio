import React from 'react';
import { Switch, Route } from 'react-router';
import Authentication from './components/Authentication';
import Authenticate from './views/Authenticate';
import Home from './views/Home';
import LoginGoogle from './views/LoginGoogle';
import './App.css';

function App() {
  return (
    <div className="app" data-testid="app">
      <Switch>
        <Route exact path="/authenticate">
          <Authenticate />
        </Route>
        <Authentication notAuthed={<LoginGoogle />}>
          <Switch>
            <Route exact path="/">
              <Home />
            </Route>
          </Switch>
        </Authentication>
      </Switch>
    </div>
  );
}

export default App;
