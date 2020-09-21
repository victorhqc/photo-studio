import React from 'react';
import { Switch, Route } from 'react-router';
import Home from './views/Home';
import './app.css';

function App() {
  return (
    <div className="app" data-testid="app">
      <Switch>
        <Route exact path="/">
          <Home />
        </Route>
      </Switch>
    </div>
  );
}

export default App;
