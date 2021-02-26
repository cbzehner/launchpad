import React, { useState, useEffect } from 'react';

import { extractFormDataFromAuthResponse, initializeAuth } from '../utils/kratos'
import { Form, FormWrapper } from '../components';
import { MethodFlowConfig } from '../types';

export default function Registration() {
  const [data, setData] = useState<MethodFlowConfig>()

  useEffect(() => {
    const asyncEffect = async () => {
      const authResponse = await initializeAuth('registration')
      const formData = extractFormDataFromAuthResponse(authResponse)
      // TODO: Guard with Typeguard rather than dangerously casting.
      setData(formData as MethodFlowConfig)
    }

    // TODO: Remove ?flow=<flowId> from the URL after the useEffect hook completes.
    asyncEffect()
  }, [])

  return (
    <FormWrapper primaryText="Sign up for an account" secondaryText="sign in to your account" secondaryUrl="http://127.0.0.1:4433/self-service/login/browser">
      <Form data={data} actionLabel="Sign up" />
    </FormWrapper>
  )
}
