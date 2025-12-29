import { createBrowserRouter } from "react-router-dom";

import { Home } from "./Home";

export const router = createBrowserRouter([
  {
    id: "root",
    path: "/",
    children: [
      {
        path: "/",
        index: true,
        Component: Home,
      },
    ],
  },
]);
