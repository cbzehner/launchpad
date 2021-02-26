import React from 'react';
import { Redirect, Route, Switch } from "react-router-dom";

import Home from './Home';

export default function AuthenticatedPages() {
  return (
    <Switch>
      <Route exact path="/">
        <Home />
      </Route>
      <Redirect to="/auth/login" /> {/* Redirect uncaught routes to login page */}
    </Switch>
  );
}
