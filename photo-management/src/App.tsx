import React, { useEffect } from 'react';
import { connect } from 'react-redux';
import { Switch, Route } from 'react-router';
import { checkCredentials } from './store/auth';
import Authentication from './components/Authentication';
import Authenticate from './views/Authenticate';
import Home from './views/Home';
import LoginGoogle from './views/LoginGoogle';
import './App.css';

function App({ checkCredentials }: Props) {
  useEffect(() => {
    checkCredentials();
  }, [checkCredentials]);

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

const mapDispatchToProps = {
  checkCredentials,
};

type Props = typeof mapDispatchToProps;

export default connect(null, mapDispatchToProps)(App);
