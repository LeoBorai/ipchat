import { RouterProvider } from "react-router-dom";

import { ChatWebSocketProvider } from "./contexts/ChatWebSocketContext";
import { NodeContextProvider } from "./contexts/NodeContext";
import { router } from "./router";

import type { JSX } from "react";
import { UIContextProvider } from "./contexts/UIContext";

export function App(): JSX.Element {
  return (
    <UIContextProvider>
      <NodeContextProvider>
        <ChatWebSocketProvider>
          <RouterProvider router={router} />
        </ChatWebSocketProvider>
      </NodeContextProvider>
    </UIContextProvider>
  );
}
