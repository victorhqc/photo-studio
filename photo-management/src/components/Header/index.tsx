import React, { FC } from 'react';
import { connect } from 'react-redux';
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
