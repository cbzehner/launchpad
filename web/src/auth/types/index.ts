import {
  LoginFlow,
  LoginFlowMethodConfig,
  RecoveryFlow,
  RecoveryFlowMethodConfig,
  RegistrationFlow,
  RegistrationFlowMethodConfig,
  SettingsFlow,
  SettingsFlowMethodConfig,
  VerificationFlow,
  VerificationFlowMethodConfig,
} from "@ory/kratos-client";

export type AutocompleteHints =
  | "off"
  | "on"
  | "name"
  | "given-name"
  | "family-name"
  | "email"
  | "username"
  | "new-password"
  | "current-password"
  | "one-time-code"
  | "organization-title"
  | "street-address"
  | "address-line1"
  | "address-line2"
  | "address-line3"
  | "address-level1"
  | "address-level2"
  | "address-level3"
  | "address-level4"
  | "country"
  | "country-name"
  | "postal-code"
  | "bday"
  | "bday-day"
  | "bday-month"
  | "bday-year"
  | "sex"
  | "tel";
export type AuthFlow =
  | LoginFlow
  | RecoveryFlow
  | RegistrationFlow
  | SettingsFlow
  | VerificationFlow;
export type MethodFlowConfig =
  | LoginFlowMethodConfig
  | RecoveryFlowMethodConfig
  | RegistrationFlowMethodConfig
  | SettingsFlowMethodConfig
  | VerificationFlowMethodConfig;
export type FlowType =
  | "login"
  | "recovery"
  | "registration"
  | "settings"
  | "verification";
export type FieldName =
  | "csrf_token"
  | "identifier"
  | "password"
  | "traits.email"
  | "traits.name.preferred"
  | "traits.name.full";
