import { AxiosResponse } from 'axios';
import { Configuration, PublicApi } from "@ory/kratos-client"

import { AuthFlow, FlowType, MethodFlowConfig } from '../types';

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

    let authRequest = requestForFlowType(authService, flowType, flowId)

    try {
      const authResponse = await authRequest
      if (authResponse.status !== 200) return reject(authResponse.data)
      return resolve(authResponse.data)
    } catch (error) {
      console.warn(error)
    }
  })
}

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
