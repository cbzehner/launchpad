import React from 'react';
import { classnames } from 'tailwindcss-classnames';
import { FormField as FormFieldType, Message as MessageType } from "@ory/kratos-client"
import sortBy from 'lodash/sortBy'

import { AutocompleteHints, FieldName, MethodFlowConfig } from '../types'

type EnhancedFormField = FormFieldType & {
  displayName?: string,
  autoComplete?: AutocompleteHints,
  autoFocus: boolean,
  order: number
}

type Props = {
  data?: MethodFlowConfig
  actionLabel?: string
}

// Auth form which takes an auth flow configuration from Kratos and renders it as a form.
// The form fields are enhanced with extra properties that are specific to UI.
export default function Form({ data, actionLabel = "Submit" }: Props) {
  if (!data) return null;

  const fields = enhanceFields(data.fields)

  return (
    <form method={data.method} action={data.action} className="space-y-6">
      {fields.map(field => <FormField key={field.name} field={field} />)}
      <div>
        <button type="submit" className="flex justify-center w-full px-4 py-2 text-sm font-medium text-white bg-indigo-600 border border-transparent rounded-md shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
          {actionLabel}
        </button>
      </div>
    </form>
  )
}

// An individual field in the auth form.
function FormField({ field }: { field: EnhancedFormField }) {
  const { messages = [] } = field

  return (
    <fieldset className={classnames({ "hidden": field.type === "hidden" })}>
      {field.displayName && <label htmlFor={field.name} className="block text-sm font-medium text-gray-700">
        {field.displayName}
      </label>}
      <div className="mt-1">
        <input
          type={field.type}
          name={field.name}
          defaultValue={field.value as unknown as string} // TODO: Fix dangerous cast. Why is the value is typed as an object by the auth service?
          required={field.required}
          autoComplete={field.autoComplete}
          // eslint-disable-next-line jsx-a11y/no-autofocus
          autoFocus={field.autoFocus} // TODO: Revisit the decision to include autoFocus.
          className="block w-full px-3 py-2 placeholder-gray-400 border border-gray-300 rounded-md shadow-sm appearance-none focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
        />
      </div>
      {messages.map(message => <Message key={message.id} message={message} />)}
    </fieldset>
  )
}

// A message provided by the auth service for display purposes.
function Message({ message }: { message: MessageType }) {
  // TODO: Convert className to use TailwindCSS classes
  return <div className={`message ${message.type}`}>{message.text}</div>
}

// Enhance the form fields with additional properties for rendering the UI and provide a stable ordering
// independent of the field order provided by the authentication service.
const enhanceFields = (fields: Array<FormFieldType>): Array<EnhancedFormField> => {
  let enhancedFields = fields.map(field => {
    return {
      displayName: displayName(field.name as FieldName), // TODO: Fix dangerous cast.
      order: fieldWeights(field.name as FieldName), // TODO: Fix dangerous cast.
      autoComplete: autoCompleteHints(field.name as FieldName), // TODO: Fix dangerous cast.
      autoFocus: false,
      ...field
    }
  })
  enhancedFields = sortBy(enhancedFields, ['order'])
  enhancedFields[0].autoFocus = true; // The first form field should grab focus.

  return enhancedFields
}

// Translate the supported field names into human readable values.
const displayName = (name: FieldName): string => {
  switch (name) {
    case 'password':
      return 'Password';
    case 'identifier':
    case 'traits.email':
      return 'Email address';
    case 'traits.name.preferred':
      return 'Preferred name';
    case 'traits.name.full':
      return 'Full name';
    case 'csrf_token':
      return 'Hidden value'
  }
}

// Assign weights to the various field names that can be used to provide a stable sort order when displaying the authentication form.
const fieldWeights = (name: FieldName): number => {
  switch (name) {
    case 'traits.name.preferred':
      return 10
    case 'traits.name.full':
      return 20
    case 'identifier':
    case 'traits.email':
      return 30
    case 'password':
      return 40
    case 'csrf_token':
      return 100
    default:
      return -1
  }
}

// Provide autocompletion hints to guide filling in the form fields.
const autoCompleteHints = (name: FieldName): AutocompleteHints => {
  switch (name) {
    case 'password':
      return 'current-password'
    case 'identifier':
    case 'traits.email':
      return 'email'
    case 'traits.name.preferred':
      return 'given-name'
    case 'traits.name.full':
      return 'family-name'
    case 'csrf_token':
      return 'off'
  }
}
