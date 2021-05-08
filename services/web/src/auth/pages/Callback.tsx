import { useEffect } from "react";
import { PublicApi, Configuration } from "@ory/kratos-client";
import { baseUrl as KratosBaseUrl } from "../utils/kratos";

export const LSK_IS_AUTHENTICATED = "kratos.isAuthenticated";

export const LSK_IS_AUTHENTICATED_REFERER = "kratos.referer";

export const getAuthenticatedReferer = () =>
  localStorage.getItem(LSK_IS_AUTHENTICATED_REFERER);

export const setAuthenticatedReferer = (location: string) =>
  localStorage.setItem(LSK_IS_AUTHENTICATED_REFERER, location);

export const unsetAuthenticatedReferer = () =>
  localStorage.removeItem(LSK_IS_AUTHENTICATED_REFERER);

export const isAuthenticated = () =>
  localStorage.getItem(LSK_IS_AUTHENTICATED) === "true";

export const setAuthenticated = () =>
  localStorage.setItem(LSK_IS_AUTHENTICATED, "true");

export const unsetAuthenticated = () =>
  localStorage.removeItem(LSK_IS_AUTHENTICATED);

export default function Callback() {
  const authService = new PublicApi(
    new Configuration({ basePath: KratosBaseUrl })
  );
  useEffect(() => {
    authService
      .whoami()
      .then(() => {
        setAuthenticated();
        unsetAuthenticatedReferer();
        window.location.href = getAuthenticatedReferer() || "/";
      })
      .catch((error) => {
        unsetAuthenticated();
        unsetAuthenticatedReferer();
        console.log(error);
      });
  });

  return null;
}
