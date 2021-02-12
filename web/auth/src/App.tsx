import React from 'react';
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";

import Login from './pages/Login';

import './App.css';

function App() {
  return (
    <div className="App">
      <Router basename="/auth">
        <Switch>
          <Route exact path="/" render={() => <div>Hello world!</div>} />
          <Route path="/login" component={Login} />
        </Switch>
      </Router>
    </div>
  );
}

export default App;
