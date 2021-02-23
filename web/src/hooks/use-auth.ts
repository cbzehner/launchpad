import { AxiosResponse } from 'axios';
import {
  Configuration, LoginFlow, LoginFlowMethodConfig, PublicApi, RecoveryFlow, RecoveryFlowMethodConfig,
  RegistrationFlow, RegistrationFlowMethodConfig, SettingsFlow, SettingsFlowMethodConfig, VerificationFlow, VerificationFlowMethodConfig
} from "@ory/kratos-client"

export type AuthFlow = LoginFlow | RecoveryFlow | RegistrationFlow | SettingsFlow | VerificationFlow
export type MethodFlowConfig = LoginFlowMethodConfig | RecoveryFlowMethodConfig | RegistrationFlowMethodConfig | SettingsFlowMethodConfig | VerificationFlowMethodConfig
type FlowType = "login" | "recovery" | "registration" | "settings" | "verification"
// TODO: Change to using "Preferred Name" & "Full Name" rather than "First Name" & "Last Name"
export type FieldName = "csrf_token" | "identifier" | "password" | "traits.email" | "traits.name.first" | "traits.name.last"

const REDIRECT_URL = "http://127.0.0.1:4455/auth/callback"
const KRATOS_BROWSER_URL = "http://127.0.0.1:4433"

// Initialize the requested authentication flow.
export const initializeAuth = (flowType: FlowType): Promise<AuthFlow> => {
  const authService = new PublicApi(new Configuration({ basePath: KRATOS_BROWSER_URL }))

  return new Promise(async (resolve, reject) => {
    // Fetch the flow id set by the Auth service from the browser URL.
    const SELF_SERVICE_URL = `http://127.0.0.1:4433/self-service/${flowType}/browser`
    const params = new URLSearchParams(window.location.search)
    // It no flow parameter is present, redirect the user to the Auth service to initialize the auth flow.
    if (!params.has('flow')) return window.location.href = SELF_SERVICE_URL
    const flowId = params.get("flow")
    if (!flowId) return window.location.href = SELF_SERVICE_URL

    // let endpoint = endpointForFlowType(flowType)
    let authRequest = requestForFlowType(authService, flowType, flowId)

    try {
      const authResponse = await authRequest
      if (authResponse.status !== 200) return reject(authResponse.data)
      return resolve(authResponse.data)
    } catch (error) {
      console.warn(error)
      // return window.location.href = endpoint!.toString()
      // return window.location.href = '/'
    }
  })
}

//
// const endpointForFlowType = (flowType: FlowType): URL => {
//   switch (flowType) {
//     case 'login':
//       return new URL(`${KRATOS_BROWSER_URL}/self-service/browser/flows/${flowType}?return_to=${REDIRECT_URL}`)
//     case 'recovery':
//       return new URL(`${KRATOS_BROWSER_URL}/self-service/browser/flows/${flowType}`)
//     case 'registration':
//       return new URL(`${KRATOS_BROWSER_URL}/self-service/browser/flows/${flowType}?return_to=${REDIRECT_URL}`)
//     case 'settings':
//       return new URL(`${KRATOS_BROWSER_URL}/self-service/browser/flows/${flowType}`)
//     case 'verification':
//       return new URL(`${KRATOS_BROWSER_URL}/self-service/browser/flows/${flowType}/init/email`)
//   }
// }

// Build the request to the authentication service to fetch the information needed for the specific login flow.
const requestForFlowType = (authService: PublicApi, flowType: FlowType, flowId: string): Promise<AxiosResponse<AuthFlow>> => {
  switch (flowType) {
    case 'login':
      return authService.getSelfServiceLoginFlow(flowId)
    case 'recovery':
      return authService.getSelfServiceRecoveryFlow(flowId)
    case 'registration':
      return authService.getSelfServiceRegistrationFlow(flowId)
    case 'settings':
      return authService.getSelfServiceSettingsFlow(flowId)
    case 'verification':
      return authService.getSelfServiceVerificationFlow(flowId)
  }
}

// The AuthRequest contains a configuration object which is used to dynamically build the
// form submitted by the user. Only Password is supported currently.
export const extractFormDataFromAuthResponse = (data: AuthFlow): MethodFlowConfig | null => {
  if (data.methods === undefined || Object.keys(data.methods).length === 0) {
    console.error("No auth methods specified.")
    return null;
  } else if (Object.keys(data.methods).length > 1) {
    console.error(`Multiple auth methods specified: ${Object.keys(data.methods).join(" & ")}`)
    return null
  }

  const authMethod = Object.keys(data.methods)[0]
  // TODO: Support other methods like OIDC.
  if (authMethod !== "password") {
    console.error("Only password method is supported.")
    return null;
  }
  return data.methods[authMethod].config
}

  // // A very common scenario is you have a bunch of components that need to render
  // // different depending on whether the current user is logged in and sometimes
  // // call authentication methods like signin, signout, sendPasswordResetEmail,
  // // etc.
  // //
  // // This is a perfect use-case for a useAuth hook that enables any component to
  // // get the current auth state and re-render if it changes. Rather than have each
  // // instance of the useAuth hook fetch the current user, the hook simply calls
  // // useContext to get the data from farther up in the component tree. The real
  // // magic happens in our <ProvideAuth> component and our useProvideAuth hook
  // // which wraps all our authentication methods (in this case we're using
  // // ORY Kratos) and then uses React Context to make the current auth object
  // // available to all child components that call useAuth. Whew, that was a
  // // mouthfull...
  // //
  // // Hopefully as you read through the code below it should all make sense.
  // // Another reason I like this method is it neatly abstracts away our actual auth
  // // provider (ORY Kratos), making it super easy to change providers in the future.
  // // Source: https://usehooks.com/useAuth

  // import React, { useState, useEffect, useContext, createContext } from "react";
  // // import * as firebase from "firebase/app";
  // // import "firebase/auth";

  // // Add your Firebase credentials
  // // firebase.initializeApp({
  // //   apiKey: "",
  // //   authDomain: "",
  // //   projectId: "",
  // //   appID: ""
  // // });

  // type Auth = {
  //   user: null
  //   signin: () => void,
  //   signup: () => void,
  //   signout: () => void,
  //   sendPasswordResetEmail: () => void,
  //   confirmPasswordReset: () => void
  // }

  // const authContext = createContext<null | Auth>();

  // // Provider component that wraps your app and makes auth object ...
  // // ... available to any child component that calls useAuth().
  // export function ProvideAuth({ children }) {
  //   const auth = useProvideAuth();
  //   return <authContext.Provider value={ auth }> { children } < /authContext.Provider>;
  // }

  // // Hook for child components to get the auth object ...
  // // ... and re-render when it changes.
  // export const useAuth = () => {
  //   return useContext(authContext);
  // };

  // // Provider hook that creates auth object and handles state
  // function useProvideAuth(): Auth {
  //   const [user, setUser] = useState(null);

  //   // Wrap any Firebase methods we want to use making sure ...
  //   // ... to save the user to state.
  //   const signin = (email: string, password: string) => {
  //     return firebase
  //       .auth()
  //       .signInWithEmailAndPassword(email, password)
  //       .then(response => {
  //         setUser(response.user);
  //         return response.user;
  //       });
  //   };

  //   const signup = (email, password) => {
  //     return firebase
  //       .auth()
  //       .createUserWithEmailAndPassword(email, password)
  //       .then(response => {
  //         setUser(response.user);
  //         return response.user;
  //       });
  //   };

  //   const signout = () => {
  //     return firebase
  //       .auth()
  //       .signOut()
  //       .then(() => {
  //         setUser(false);
  //       });
  //   };

  //   const sendPasswordResetEmail = email => {
  //     return firebase
  //       .auth()
  //       .sendPasswordResetEmail(email)
  //       .then(() => {
  //         return true;
  //       });
  //   };

  //   const confirmPasswordReset = (code, password) => {
  //     return firebase
  //       .auth()
  //       .confirmPasswordReset(code, password)
  //       .then(() => {
  //         return true;
  //       });
  //   };

  //   // Subscribe to user on mount
  //   // Because this sets state in the callback it will cause any ...
  //   // ... component that utilizes this hook to re-render with the ...
  //   // ... latest auth object.
  //   useEffect(() => {
  //     const unsubscribe = firebase.auth().onAuthStateChanged(user => {
  //       if (user) {
  //         setUser(user);
  //       } else {
  //         setUser(false);
  //       }
  //     });

  //     // Cleanup subscription on unmount
  //     return () => unsubscribe();
  //   }, []);

  //   // Return the user object and auth methods
  //   return {
  //     user,
  //     signin,
  //     signup,
  //     signout,
  //     sendPasswordResetEmail,
  //     confirmPasswordReset
  //   };
  // }
