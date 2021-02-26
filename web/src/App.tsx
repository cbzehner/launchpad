import React, { lazy, Suspense } from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";

// import { ProvideAuth } from "./hooks/use-auth"
import Loading from "./components/Loading";

import "./App.css";

const AuthPages = lazy(() => import("./auth/pages"));
const LoggedInPages = lazy(() => import("./pages"));

function App() {
  return (
    <div className="App">
      {/* <ProvideAuth> */}
      <Router>
        <Suspense fallback={<Loading />}>
          <Switch>
            <Route path="/auth">
              <AuthPages />
            </Route>
            <Route>
              <LoggedInPages />
            </Route>
          </Switch>
        </Suspense>
      </Router>
      {/* </ProvideAuth> */}
    </div>
  );
}

export default App;
