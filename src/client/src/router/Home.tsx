import { useState, useEffect, useRef } from "react";
import {
  Wifi,
  Users,
  MessageSquare,
  Plus,
  Send,
  X,
  User,
  RefreshCw,
} from "lucide-react";

import { nodeInfo } from "../lib/api";

export function Home() {
  const [username, setUsername] = useState("");
  const [isSetup, setIsSetup] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const [serverUrl, setServerUrl] = useState("ws://localhost:8080");

  const [myRooms, setMyRooms] = useState([]);
  const [discoveredPeers, setDiscoveredPeers] = useState([]);
  const [activeRoom, setActiveRoom] = useState(null);
  const [messages, setMessages] = useState({});
  const [newMessage, setNewMessage] = useState("");
  const [newRoomName, setNewRoomName] = useState("");
  const [showCreateRoom, setShowCreateRoom] = useState(false);
  const [connectionStatus, setConnectionStatus] = useState("Disconnected");

  const ws = useRef(null);
  const reconnectTimeout = useRef(null);

  // WebSocket connection management
  const connectWebSocket = () => {
    try {
      ws.current = new WebSocket(serverUrl);

      ws.current.onopen = () => {
        console.log("Connected to server");
        setIsConnected(true);
        setConnectionStatus("Connected");

        // Request initial data
        requestPeerList();
        requestRoomList();
      };

      ws.current.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data);
          handleServerMessage(message);
        } catch (error) {
          console.error("Failed to parse message:", error);
        }
      };

      ws.current.onerror = (error) => {
        console.error("WebSocket error:", error);
        setConnectionStatus("Error");
      };

      ws.current.onclose = () => {
        console.log("Disconnected from server");
        setIsConnected(false);
        setConnectionStatus("Disconnected");

        // Attempt to reconnect after 3 seconds
        reconnectTimeout.current = setTimeout(() => {
          if (isSetup) {
            setConnectionStatus("Reconnecting...");
            connectWebSocket();
          }
        }, 3000);
      };
    } catch (error) {
      console.error("Failed to connect:", error);
      setConnectionStatus("Connection Failed");
    }
  };

  // Handle messages from server
  const handleServerMessage = (message) => {
    console.log("Received:", message);

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
        message.rooms.forEach((room) => {
          if (!messages[room.id]) {
            setMessages((prev) => ({ ...prev, [room.id]: [] }));
          }
        });
        break;

      case "Error":
        console.error("Server error:", message.message);
        break;

      default:
        console.log("Unknown message type:", message.type);
    }
  };

  // Send message to server
  const sendToServer = (message) => {
    if (ws.current && ws.current.readyState === WebSocket.OPEN) {
      ws.current.send(JSON.stringify(message));
    } else {
      console.error("WebSocket is not connected");
    }
  };

  // API functions
  const requestPeerList = () => {
    sendToServer({ type: "ListPeers" });
  };

  const requestRoomList = () => {
    sendToServer({ type: "ListRooms" });
  };

  const createRoom = () => {
    if (newRoomName.trim()) {
      sendToServer({
        type: "CreateRoom",
        name: newRoomName,
      });
      setNewRoomName("");
      setShowCreateRoom(false);
    }
  };

  const joinRoom = (roomId, peerIp) => {
    sendToServer({
      type: "JoinRoom",
      room_id: roomId,
      peer_ip: peerIp,
    });
  };

  const sendMessage = () => {
    if (newMessage.trim() && activeRoom) {
      sendToServer({
        type: "SendMessage",
        room_id: activeRoom.id,
        content: newMessage,
      });
      setNewMessage("");
    }
  };

  // Setup handler
  const handleSetup = () => {
    if (username.trim()) {
      setIsSetup(true);
      setConnectionStatus("Connecting...");
      connectWebSocket();
    }
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
      if (ws.current) {
        ws.current.close();
      }
      if (reconnectTimeout.current) {
        clearTimeout(reconnectTimeout.current);
      }
    };
  }, []);

  useEffect(() => {
   	nodeInfo({
      baseUrl: '',
      fetch
    });
  }, []);

  // Format timestamp
  const formatTime = (timestamp) => {
    const date = new Date(timestamp);
    return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  };

  const getRoomMessages = () => {
    return activeRoom ? messages[activeRoom.id] || [] : [];
  };

  if (!isSetup) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
        <div className="bg-white p-8 rounded-2xl shadow-xl w-96">
          <div className="flex items-center justify-center mb-6">
            <div className="bg-indigo-100 p-3 rounded-full">
              <Wifi className="w-8 h-8 text-indigo-600" />
            </div>
          </div>
          <h1 className="text-2xl font-bold text-center mb-2 text-gray-800">
            Local Network Chat
          </h1>
          <p className="text-center text-gray-600 mb-6 text-sm">
            Connect to Rust P2P Server
          </p>

          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Server Address
              </label>
              <input
                type="text"
                placeholder="ws://localhost:8080"
                value={serverUrl}
                onChange={(e) => setServerUrl(e.target.value)}
                className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Username
              </label>
              <input
                type="text"
                placeholder="Enter your username"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                onKeyPress={(e) =>
                  e.key === "Enter" && username.trim() && handleSetup()
                }
                className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <button
              onClick={handleSetup}
              disabled={!username.trim()}
              className="w-full bg-indigo-600 text-white py-3 rounded-lg font-semibold hover:bg-indigo-700 transition disabled:bg-gray-400 disabled:cursor-not-allowed"
            >
              Connect to Network
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-screen bg-gray-100">
      {/* Sidebar */}
      <div className="w-80 bg-white border-r border-gray-200 flex flex-col">
        {/* User Info */}
        <div className="p-4 border-b border-gray-200 bg-indigo-50">
          <div className="flex items-center justify-between mb-2">
            <div className="flex items-center gap-3">
              <div className="bg-indigo-600 p-2 rounded-full">
                <User className="w-5 h-5 text-white" />
              </div>
              <div>
                <p className="font-semibold text-gray-800">{username}</p>
                <p className="text-xs text-gray-600">{serverUrl}</p>
              </div>
            </div>
            <button
              onClick={requestPeerList}
              className="p-2 hover:bg-indigo-100 rounded-full transition"
              title="Refresh peers"
            >
              <RefreshCw className="w-4 h-4 text-indigo-600" />
            </button>
          </div>
          <div className="flex items-center gap-2">
            <div
              className={`w-2 h-2 rounded-full ${isConnected ? "bg-green-500" : "bg-red-500"}`}
            ></div>
            <p className="text-xs text-gray-600">{connectionStatus}</p>
          </div>
        </div>

        {/* My Rooms */}
        <div className="flex-1 overflow-y-auto">
          <div className="p-4">
            <div className="flex items-center justify-between mb-3">
              <h3 className="font-semibold text-gray-700 flex items-center gap-2">
                <MessageSquare className="w-4 h-4" />
                My Rooms
              </h3>
              <button
                onClick={() => setShowCreateRoom(!showCreateRoom)}
                className="p-1 hover:bg-gray-100 rounded"
              >
                <Plus className="w-4 h-4 text-indigo-600" />
              </button>
            </div>

            {showCreateRoom && (
              <div className="mb-3 flex gap-2">
                <input
                  type="text"
                  placeholder="Room name"
                  value={newRoomName}
                  onChange={(e) => setNewRoomName(e.target.value)}
                  onKeyPress={(e) => e.key === "Enter" && createRoom()}
                  className="flex-1 px-3 py-2 border border-gray-300 rounded text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
                <button
                  onClick={createRoom}
                  className="px-3 py-2 bg-indigo-600 text-white rounded text-sm hover:bg-indigo-700"
                >
                  Add
                </button>
              </div>
            )}

            {myRooms.length === 0 ? (
              <p className="text-sm text-gray-500">No rooms yet. Create one!</p>
            ) : (
              myRooms.map((room) => (
                <button
                  key={room.id}
                  onClick={() => setActiveRoom(room)}
                  className={`w-full text-left p-3 rounded-lg mb-2 transition ${
                    activeRoom?.id === room.id
                      ? "bg-indigo-100 border border-indigo-300"
                      : "hover:bg-gray-50 border border-transparent"
                  }`}
                >
                  <p className="font-medium text-gray-800">{room.name}</p>
                  <p className="text-xs text-gray-500">
                    {room.participants?.length || 0} participant(s)
                  </p>
                </button>
              ))
            )}
          </div>

          {/* Discovered Peers */}
          <div className="p-4 border-t border-gray-200">
            <h3 className="font-semibold text-gray-700 mb-3 flex items-center gap-2">
              <Users className="w-4 h-4" />
              Network Peers ({discoveredPeers.length})
            </h3>
            {discoveredPeers.length === 0 ? (
              <p className="text-sm text-gray-500">
                {isConnected
                  ? "No peers discovered yet..."
                  : "Connect to see peers"}
              </p>
            ) : (
              discoveredPeers.map((peer) => (
                <div key={peer.ip} className="mb-4 p-3 bg-gray-50 rounded-lg">
                  <div className="flex items-center gap-2 mb-2">
                    <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                    <p className="font-medium text-sm text-gray-800">
                      {peer.username}
                    </p>
                  </div>
                  <p className="text-xs text-gray-500 font-mono mb-2">
                    {peer.ip}
                  </p>
                  {console.log(peer)}
                  {/*{peer.rooms && peer.rooms.length > 0 ? (
                    <div className="space-y-1">
                      {peer.rooms.map((roomName) => (
                        <button
                          key={`${peer.ip}_${roomName}`}
                          onClick={() => joinRoom(roomName, peer.ip)}
                          className="w-full text-left px-2 py-1 text-xs bg-white hover:bg-indigo-50 rounded border border-gray-200 hover:border-indigo-300 transition"
                        >
                          {roomName}
                        </button>
                      ))}
                    </div>
                  ) : (
                    <p className="text-xs text-gray-400">No rooms</p>
                  )}*/}
                </div>
              ))
            )}
          </div>
        </div>
      </div>

      {/* Chat Area */}
      <div className="flex-1 flex flex-col">
        {activeRoom ? (
          <>
            {/* Chat Header */}
            <div className="bg-white border-b border-gray-200 p-4">
              <h2 className="font-bold text-lg text-gray-800">
                {activeRoom.name}
              </h2>
              <p className="text-sm text-gray-500">
                {activeRoom.host ? `Hosted by ${activeRoom.host}` : "Your room"}
              </p>
            </div>

            {/* Messages */}
            <div className="flex-1 overflow-y-auto p-4 space-y-3">
              {getRoomMessages().map((msg, idx) => (
                <div
                  key={idx}
                  className={`flex ${
                    msg.sender === username ? "justify-end" : "justify-start"
                  }`}
                >
                  <div
                    className={`max-w-xs px-4 py-2 rounded-lg ${
                      msg.sender === "System"
                        ? "bg-gray-200 text-gray-700 text-sm italic"
                        : msg.sender === username
                          ? "bg-indigo-600 text-white"
                          : "bg-white border border-gray-200 text-gray-800"
                    }`}
                  >
                    {msg.sender !== username && msg.sender !== "System" && (
                      <p className="text-xs font-semibold mb-1 opacity-75">
                        {msg.sender}
                      </p>
                    )}
                    <p className="text-sm">{msg.content}</p>
                    <p
                      className={`text-xs mt-1 ${
                        msg.sender === username
                          ? "text-indigo-200"
                          : "text-gray-500"
                      }`}
                    >
                      {formatTime(msg.timestamp)}
                    </p>
                  </div>
                </div>
              ))}
            </div>

            {/* Message Input */}
            <div className="bg-white border-t border-gray-200 p-4">
              <div className="flex gap-2">
                <input
                  type="text"
                  placeholder={
                    isConnected ? "Type a message..." : "Connecting..."
                  }
                  value={newMessage}
                  onChange={(e) => setNewMessage(e.target.value)}
                  onKeyPress={(e) => e.key === "Enter" && sendMessage()}
                  disabled={!isConnected}
                  className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 disabled:bg-gray-100"
                />
                <button
                  onClick={sendMessage}
                  disabled={!isConnected || !newMessage.trim()}
                  className="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition disabled:bg-gray-400 disabled:cursor-not-allowed"
                >
                  <Send className="w-5 h-5" />
                </button>
              </div>
            </div>
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center text-gray-500">
            <div className="text-center">
              <MessageSquare className="w-16 h-16 mx-auto mb-4 opacity-50" />
              <p className="text-lg font-semibold mb-2">No room selected</p>
              <p className="text-sm">
                Create a room or join a peer's room to start chatting
              </p>
              {!isConnected && (
                <p className="text-sm text-red-500 mt-2">
                  Waiting for server connection...
                </p>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
