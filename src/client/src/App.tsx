import { RouterProvider } from "react-router-dom";

import { router } from "./router";

import type { JSX } from "react";

export function App(): JSX.Element {
  return <RouterProvider router={router} />;
}
