import React from 'react';
import { Redirect, Route, Switch } from "react-router-dom";

import Callback from './Callback';
import Login from './Login';
import Registration from './Registration';

export default function Auth() {
  return (
    <div className="Auth">
      <Switch>
        <Route path="/auth/login">
          <Login />
        </Route>
        <Route path="/auth/registration">
          <Registration />
        </Route>
        <Route path="/auth/callback">
          <Callback />
        </Route>
        <Redirect to="/auth/login" />
      </Switch>
    </div >
  );
}
