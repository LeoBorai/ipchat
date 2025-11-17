import { RouterProvider } from "react-router-dom";

import { router } from "./router";
import { ChatWebSocketProvider } from "./contexts/ChatWebSocketContext";

import type { JSX } from "react";
import { NodeContextProvider } from "./contexts/NodeContext";

export function App(): JSX.Element {
  return (
    <NodeContextProvider>
      <ChatWebSocketProvider>
        <RouterProvider router={router} />
      </ChatWebSocketProvider>
    </NodeContextProvider>
  );
}
