import 'whatwg-fetch';
import React from 'react';
import ReactDOM from 'react-dom';

import './main.css';

import HttpApi from './lib/api/HttpApi';
import BoardPage from './lib/components/BoardPage';

const root = document.getElementById('root');
const api = new HttpApi('/api');
const page = <BoardPage api={api} />;

ReactDOM.render(page, root);