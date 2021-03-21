import React from "react";
import { render, screen as testScreen } from "@testing-library/react";
import App from "./App";

test("renders learn react link", () => {
  render(<App />);
  const linkElement = testScreen.getByText(/learn react/i);
  expect(linkElement).toBeInTheDocument();
});
