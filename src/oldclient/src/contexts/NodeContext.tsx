import { createContext, useEffect, useState, useContext } from "react";

import { nodeInfo } from "../services/IPChat/bindings/sdk.gen";

import type { JSX } from "react";

export type Props = {
  children: JSX.Element | JSX.Element[];
};

export interface NodeInfo {
  webSocketAddr: string;
  clientIp: string;
  localIp: string;
}

export interface INodeContext {
  nodeInfo: NodeInfo | null;
  serverUrl: string;
  isLoading: boolean;
}

export const NodeContext = createContext<INodeContext | null>(null);
NodeContext.displayName = "NodeContext";

export function NodeContextProvider({ children }: Props): JSX.Element {
  const [nodeInfoData, setNodeInfoData] = useState<NodeInfo | null>(null);
  const [serverUrl, setServerUrl] = useState("ws://localhost:8080");
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const fetchNodeInfo = async () => {
      try {
        const response = await nodeInfo({
          baseUrl: "",
          fetch,
        });

        if (response.data) {
          setNodeInfoData(response.data);

          const webSocketPort = response.data.webSocketAddr.split(":")[1];

          // we are running in the same machine as the server
          if (response.data.clientIp.startsWith("127.0.0.1")) {
            setServerUrl(`ws://localhost:${webSocketPort}`);
          } else {
            const serverLocalIpAddr = response.data.localIp.split(":")[0];
            setServerUrl(`ws://${serverLocalIpAddr}:${webSocketPort}`);
          }
        }
      } catch (error) {
        console.error("Failed to fetch node info:", error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchNodeInfo();
  }, []);

  return (
    <NodeContext.Provider
      value={{
        nodeInfo: nodeInfoData,
        serverUrl,
        isLoading,
      }}
    >
      {children}
    </NodeContext.Provider>
  );
}

export function useNode(): INodeContext {
  const context = useContext(NodeContext);
  if (!context) {
    throw new Error("useNode must be used within a NodeContextProvider");
  }
  return context;
}
