import { RouterProvider } from "react-router-dom";

import { router } from "./router";
import { ChatWebSocketProvider } from "./contexts/ChatWebSocketContext";

import type { JSX } from "react";

export function App(): JSX.Element {
  return (
    <ChatWebSocketProvider>
      <RouterProvider router={router} />
    </ChatWebSocketProvider>
  );
}
