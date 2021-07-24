import React from 'react';
import ReactDOM from 'react-dom';

import './styles.scss';
import App from './app';

window.onload = () => {
  ReactDOM.render(<App />, document.getElementById('app'));
}
