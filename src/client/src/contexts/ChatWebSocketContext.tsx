import { createContext, useEffect, useState, useRef, useContext } from "react";

import { ChatWebSocketService } from "../services/ChatWebSocketService";

import type { JSX } from "react";
import type {
  ConnectionStatus,
  ServerMessage,
} from "../services/ChatWebSocketService";

export type Props = {
  children: JSX.Element | JSX.Element[];
};

export interface Room {
  id: string;
  name: string;
  host?: string;
  participants?: string[];
}

export interface Peer {
  ip: string;
  username: string;
  rooms?: string[];
}

export interface Message {
  sender: string;
  content: string;
  timestamp: string;
  room_id?: string;
}

export interface IChatWebSocketContext {
  isConnected: boolean;
  connectionStatus: ConnectionStatus;
  myRooms: Room[];
  discoveredPeers: Peer[];
  activeRoom: Room | null;
  messages: Record<string, Message[]>;
  setActiveRoom: (room: Room | null) => void;
  requestPeerList: () => void;
  requestRoomList: () => void;
  createRoom: (roomName: string) => void;
  joinRoom: (roomId: string, peerIp: string) => void;
  sendMessage: (roomId: string, content: string, sender: string) => void;
  connect: (serverUrl: string) => void;
  disconnect: () => void;
}

export const ChatWebSocketContext = createContext<IChatWebSocketContext | null>(
  null,
);
ChatWebSocketContext.displayName = "ChatWebSocketContext";

export function ChatWebSocketProvider({ children }: Props): JSX.Element {
  const [isConnected, setIsConnected] = useState(false);
  const [connectionStatus, setConnectionStatus] =
    useState<ConnectionStatus>("Disconnected");
  const [myRooms, setMyRooms] = useState<Room[]>([]);
  const [discoveredPeers, setDiscoveredPeers] = useState<Peer[]>([]);
  const [activeRoom, setActiveRoom] = useState<Room | null>(null);
  const [messages, setMessages] = useState<Record<string, Message[]>>({});
  const wsService = useRef<ChatWebSocketService | null>(null);

  // Handle messages from server
  const handleServerMessage = (message: ServerMessage) => {
    switch (message.type) {
      case "RoomCreated":
        setMyRooms((prev) => [...prev, message.room]);
        setMessages((prev) => ({ ...prev, [message.room.id]: [] }));
        break;

      case "RoomJoined":
        setActiveRoom(message.room);
        if (!messages[message.room.id]) {
          setMessages((prev) => ({
            ...prev,
            [message.room.id]: [
              {
                sender: "System",
                content: `You joined ${message.room.name}`,
                timestamp: new Date().toISOString(),
              },
            ],
          }));
        }
        break;

      case "NewMessage":
        setMessages((prev) => ({
          ...prev,
          [message.message.room_id]: [
            ...(prev[message.message.room_id] || []),
            message.message,
          ],
        }));
        break;

      case "PeerList":
        setDiscoveredPeers(message.peers);
        break;

      case "RoomList":
        setMyRooms(message.rooms);
        setMessages((prev) => {
          const updates = { ...prev };
          message.rooms.forEach((room) => {
            if (!updates[room.id]) {
              updates[room.id] = [];
            }
          });
          return updates;
        });
        break;

      case "Error":
        console.error("Server error:", message.message);
        break;

      default:
        console.log("Unknown message type:", message);
    }
  };

  // Connect to WebSocket
  const connect = (serverUrl: string) => {
    wsService.current = new ChatWebSocketService({
      serverUrl,
      onConnectionChange: (connected, status) => {
        setIsConnected(connected);
        setConnectionStatus(status);
      },
      onMessage: handleServerMessage,
      onInitialConnect: () => {
        wsService.current?.listNodes();
        wsService.current?.listRooms();
      },
    });

    wsService.current.connect();
  };

  // Disconnect from WebSocket
  const disconnect = () => {
    wsService.current?.disconnect();
  };

  // API functions
  const requestPeerList = () => {
    wsService.current?.listNodes();
  };

  const requestRoomList = () => {
    wsService.current?.listRooms();
  };

  const createRoom = (roomName: string) => {
    wsService.current?.createRoom(roomName);
  };

  const joinRoom = (roomId: string, peerIp: string) => {
    wsService.current?.joinRoom(roomId, peerIp);
  };

  const sendMessage = (roomId: string, content: string, sender: string) => {
    wsService.current?.sendMessage(roomId, content, sender);
  };

  // Periodic peer refresh
  useEffect(() => {
    if (isConnected) {
      const interval = setInterval(() => {
        requestPeerList();
      }, 5000);

      return () => clearInterval(interval);
    }
  }, [isConnected]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      wsService.current?.disconnect();
    };
  }, []);

  return (
    <ChatWebSocketContext.Provider
      value={{
        isConnected,
        connectionStatus,
        myRooms,
        discoveredPeers,
        activeRoom,
        messages,
        setActiveRoom,
        requestPeerList,
        requestRoomList,
        createRoom,
        joinRoom,
        sendMessage,
        connect,
        disconnect,
      }}
    >
      {children}
    </ChatWebSocketContext.Provider>
  );
}

export function useChatWebSocket(): IChatWebSocketContext {
  const context = useContext(ChatWebSocketContext);
  if (!context) {
    throw new Error(
      "useChatWebSocket must be used within a ChatWebSocketProvider",
    );
  }
  return context;
}
