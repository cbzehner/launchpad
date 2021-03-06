import { AxiosResponse } from "axios";
import { Configuration, PublicApi } from "@ory/kratos-client";

import { AuthFlow, FlowType, MethodFlowConfig } from "../types";

export const baseUrl = `${window.location.origin}/.ory/kratos/public`;

// Initialize the requested authentication flow.
export const initializeAuth = async (
  flowType: FlowType
): Promise<AuthFlow | null> => {
  const authService = new PublicApi(new Configuration({ basePath: baseUrl }));

  // Fetch the flow id set by the Auth service from the browser URL.
  const params = new URLSearchParams(window.location.search);
  // It no flow parameter is present, redirect the user to the Auth service to initialize the auth flow.
  if (!params.has("flow")) {
    window.location.href = selfServiceAuthUrl(flowType);
    return null;
  }
  const flowId = params.get("flow");
  if (!flowId) {
    window.location.href = selfServiceAuthUrl(flowType);
    return null;
  }

  let authRequest = requestForFlowType(authService, flowType, flowId);

  const authResponse = await authRequest;

  if (authResponse.status !== 200) {
    throw new Error(`Invalid auth response. Status: ${authResponse.status}`);
  }

  return authResponse.data;
};

export const selfServiceAuthUrl = (flowType: FlowType) =>
  `${baseUrl}/self-service/${flowType}/browser`;

// Build the request to the authentication service to fetch the information needed for the specific login flow.
const requestForFlowType = (
  authService: PublicApi,
  flowType: FlowType,
  flowId: string
): Promise<AxiosResponse<AuthFlow>> => {
  switch (flowType) {
    case "login":
      return authService.getSelfServiceLoginFlow(flowId);
    case "recovery":
      return authService.getSelfServiceRecoveryFlow(flowId);
    case "registration":
      return authService.getSelfServiceRegistrationFlow(flowId);
    case "settings":
      return authService.getSelfServiceSettingsFlow(flowId);
    case "verification":
      return authService.getSelfServiceVerificationFlow(flowId);
  }
};

// The AuthRequest contains a configuration object which is used to dynamically build the
// form submitted by the user. Only Password is supported currently.
export const extractFormDataFromAuthResponse = (
  data: AuthFlow | null
): MethodFlowConfig | null => {
  if (!data) {
    console.error("No data");
    return null;
  } else if (
    data.methods === undefined ||
    Object.keys(data.methods).length === 0
  ) {
    console.error("No auth methods specified.");
    return null;
  } else if (Object.keys(data.methods).length > 1) {
    console.error(
      `Multiple auth methods specified: ${Object.keys(data.methods).join(
        " & "
      )}`
    );
    return null;
  }

  const authMethod = Object.keys(data.methods)[0];
  // TODO: Support other methods like OIDC.
  if (authMethod !== "password") {
    console.error("Only password method is supported.");
    return null;
  }
  return data.methods[authMethod].config;
};
