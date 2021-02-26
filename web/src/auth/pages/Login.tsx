import React, { useState, useEffect } from 'react';

import { extractFormDataFromAuthResponse, initializeAuth } from '../utils/kratos'
import { Form, FormWrapper } from '../components';
import { MethodFlowConfig } from '../types'

export default function Login() {
  const [data, setData] = useState<MethodFlowConfig>()

  useEffect(() => {
    const asyncEffect = async () => {
      const authResponse = await initializeAuth('login')
      const formData = extractFormDataFromAuthResponse(authResponse)
      // TODO: Guard with Typeguard rather than dangerously casting.
      setData(formData as MethodFlowConfig)
    }

    // TODO: Remove ?flow=<flowId> from the URL after the useEffect hook completes.
    asyncEffect()
  }, [])

  return (
    <FormWrapper primaryText="Sign in to your account" secondaryText="sign up for an account" secondaryUrl="http://127.0.0.1:4433/self-service/registration/browser">
      <Form data={data} actionLabel="Sign in" />
    </FormWrapper>
  )
}
