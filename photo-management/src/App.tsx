import React from 'react';
import { Switch, Route } from 'react-router';
import Authentication from './components/Authentication';
import Home from './views/Home';
import LoginGoogle from './views/LoginGoogle';
import './app.css';

function App() {
  return (
    <div className="app" data-testid="app">
      <Authentication notAuthed={<LoginGoogle />}>
        <Switch>
          <Route exact path="/">
            <Home />
          </Route>
        </Switch>
      </Authentication>
    </div>
  );
}

export default App;
