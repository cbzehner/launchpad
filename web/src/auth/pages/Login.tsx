import React, { useState, useEffect } from "react";

import {
  extractFormDataFromAuthResponse,
  initializeAuth,
  selfServiceAuthUrl,
} from "../utils/kratos";
import { Form, FormWrapper } from "../components";
import { MethodFlowConfig } from "../types";

export default function Login() {
  const [data, setData] = useState<MethodFlowConfig>();

  useEffect(() => {
    const asyncEffect = async () => {
      const authResponse = await initializeAuth("login");
      const formData = extractFormDataFromAuthResponse(authResponse);
      // TODO: Guard with Typeguard rather than dangerously casting.
      setData(formData as MethodFlowConfig);
    };

    // TODO: Remove ?flow=<flowId> from the URL after the useEffect hook completes.
    asyncEffect();
  }, []);

  return (
    <FormWrapper
      primaryText="Sign in to your account"
      secondaryText="sign up for an account"
      secondaryUrl={selfServiceAuthUrl("registration")}
    >
      <Form data={data} actionLabel="Sign in" />
    </FormWrapper>
  );
}
