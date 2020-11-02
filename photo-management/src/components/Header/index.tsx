import React, { FC } from 'react';
import { connect } from 'react-redux';
import { Link } from 'react-router-dom';
import { SignOutIcon } from '@primer/octicons-react';
import { ApplicationState } from '../../store';
import { selectAuthenticatedUser, logout } from '../../store/auth';
import './styles.css';

const Header: FC<Props> = ({ user, logout }) => (
  <header className="header">
    <div className="header__user-info">
      <img className="header__img" src={user.picture} alt="Profile" referrerPolicy="no-referrer" />
      <p className="header__email">{user.email}</p>
    </div>
    <nav className="header__nav">
      <ul className="header__nav__ul">
        <li>
          <Link to="/">Albums</Link>
        </li>
        <li>
          <Link to="/book_me">Book Me</Link>
        </li>
      </ul>
    </nav>
    <button className="header__logout" onClick={() => logout()}>
      <span className="header__logout-icon">
        <SignOutIcon verticalAlign="middle" />
      </span>
      Logout
    </button>
  </header>
);

const mapDispatchToProps = {
  logout,
};

const mapStateToProps = (state: ApplicationState) => ({
  user: selectAuthenticatedUser(state),
});

type Props = ReturnType<typeof mapStateToProps> & typeof mapDispatchToProps;

export default connect(mapStateToProps, mapDispatchToProps)(Header);
