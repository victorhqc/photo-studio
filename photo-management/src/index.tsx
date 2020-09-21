import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import Root from './components/Root';

ReactDOM.render(
  <React.StrictMode>
    <Root>
      <App />
    </Root>
  </React.StrictMode>,
  document.getElementById('root')
);
