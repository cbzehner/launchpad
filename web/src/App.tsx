import React, { lazy, Suspense } from 'react';
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";

// import { ProvideAuth } from "./hooks/use-auth"
import Loading from './components/Loading';

import './App.css';

const Auth = lazy(() => import('./pages/auth'))
const AuthenticatedPages = lazy(() => import('./pages'))

function App() {
  return (
    <div className="App">
      {/* <ProvideAuth> */}
      <Router>
        <Suspense fallback={<Loading />} >
          <Switch>
            <Route path="/auth" >
              <Auth />
            </Route>
            <Route>
              <AuthenticatedPages />
            </Route>
          </Switch>
        </Suspense>
      </Router >
      {/* </ProvideAuth> */}
    </div >
  );
}

export default App;
