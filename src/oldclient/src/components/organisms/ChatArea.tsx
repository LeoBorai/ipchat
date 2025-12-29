import { useState } from "react";
import { MessageSquare, Send } from "lucide-react";

import { useChatWebSocket } from "../../contexts/ChatWebSocketContext";

interface ChatAreaProps {
  username: string;
}

export function ChatArea({ username }: ChatAreaProps) {
  const [newMessage, setNewMessage] = useState("");
  const {
    isConnected,
    activeRoom,
    messages,
    sendMessage: contextSendMessage,
  } = useChatWebSocket();

  const handleSendMessage = () => {
    if (newMessage.trim() && activeRoom) {
      contextSendMessage(activeRoom.id, newMessage, username);
      setNewMessage("");
    }
  };

  const formatTime = (timestamp: string | number): string => {
    const date = new Date(typeof timestamp === "string" ? parseInt(timestamp, 10) : timestamp);
    return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  };

  const getRoomMessages = () => {
    return activeRoom ? messages[activeRoom.id] || [] : [];
  };

  return (
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
  );
}
