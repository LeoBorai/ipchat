import { useState } from "react";
import {
  Users,
  MessageSquare,
  Plus,
  Send,
  User,
  RefreshCw,
} from "lucide-react";

import { SignInForm } from "../components/atoms/SignInForm";
import { useChatWebSocket } from "../contexts/ChatWebSocketContext";
import { useNode } from "../contexts/NodeContext";

export function Home() {
  const [username, setUsername] = useState("");
  const [isSetup, setIsSetup] = useState(false);
  const [newMessage, setNewMessage] = useState("");
  const [newRoomName, setNewRoomName] = useState("");
  const [showCreateRoom, setShowCreateRoom] = useState(false);
  const { serverUrl } = useNode();
  const {
    isConnected,
    connectionStatus,
    myRooms,
    discoveredPeers,
    activeRoom,
    messages,
    setActiveRoom,
    requestPeerList,
    createRoom: contextCreateRoom,
    sendMessage: contextSendMessage,
    connect,
  } = useChatWebSocket();

  const handleSetup = (user: string) => {
    setUsername(user);
    setIsSetup(true);
    connect(serverUrl);
  };

  const handleCreateRoom = () => {
    if (newRoomName.trim()) {
      contextCreateRoom(newRoomName);
      setNewRoomName("");
      setShowCreateRoom(false);
    }
  };

  const handleSendMessage = () => {
    if (newMessage.trim() && activeRoom) {
      contextSendMessage(activeRoom.id, newMessage, username);
      setNewMessage("");
    }
  };

  const formatTime = (timestamp: number): string => {
    const date = new Date(timestamp);
    return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  };

  const getRoomMessages = () => {
    return activeRoom ? messages[activeRoom.id] || [] : [];
  };

  if (!isSetup) {
    return <SignInForm onSubmit={handleSetup} initialServerUrl={serverUrl} />;
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
                  onKeyPress={(e) => e.key === "Enter" && handleCreateRoom()}
                  className="flex-1 px-3 py-2 border border-gray-300 rounded text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
                <button
                  onClick={handleCreateRoom}
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
                      {formatTime(parseInt(msg.timestamp, 10))}
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
                  onKeyPress={(e) => e.key === "Enter" && handleSendMessage()}
                  disabled={!isConnected}
                  className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 disabled:bg-gray-100"
                />
                <button
                  onClick={handleSendMessage}
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
