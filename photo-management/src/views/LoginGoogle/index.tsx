import React, { FC } from 'react';
import { getApiUrl } from '../../utils/env';
import './styles.css';

const LoginGoogle: FC = () => {
  return (
    <div id="login-google" data-testid="login-google">
      <h1 className="title">To use application you need to authenticate first</h1>
      <a href={`${getApiUrl()}/google/authorize`} className="login-goggle-link">
        Login with google
      </a>
    </div>
  );
};

export default LoginGoogle;
