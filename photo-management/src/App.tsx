import React, { useEffect } from 'react';
import { connect } from 'react-redux';
import { Switch, Route } from 'react-router';
import { checkCredentials } from './store/auth';
import Authentication from './components/Authentication';
import Header from './components/Header';
import Authenticate from './views/Authenticate';
import Album from './views/Album';
import Albums from './views/Albums';
import BookMe from './views/BookMe';
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
          <>
            <Header />
            <Switch>
              <Route exact path="/">
                <Albums />
              </Route>
              <Route exact path="/album/:id">
                <Album />
              </Route>
              <Route exact path="/book_me">
                <BookMe />
              </Route>
            </Switch>
          </>
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
