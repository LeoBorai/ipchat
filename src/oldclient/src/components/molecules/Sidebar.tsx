import { useState } from "react";
import {
  Users,
  MessageSquare,
  Plus,
  User,
  RefreshCw,
  PanelLeft,
} from "lucide-react";

import { useChatWebSocket } from "../../contexts/ChatWebSocketContext";
import { useNode } from "../../contexts/NodeContext";
import { useUI } from "../../contexts/UIContext";

interface SidebarProps {
  username: string;
}

export function Sidebar({ username }: SidebarProps) {
  const [newRoomName, setNewRoomName] = useState("");
  const [showCreateRoom, setShowCreateRoom] = useState(false);
  const { serverUrl } = useNode();
  const { isSidebarOpen, openSidebar, closeSidebar } = useUI();

  const {
    isConnected,
    connectionStatus,
    myRooms,
    discoveredPeers,
    activeRoom,
    setActiveRoom,
    requestPeerList,
    createRoom: contextCreateRoom,
  } = useChatWebSocket();

  const handleCreateRoom = () => {
    if (newRoomName.trim()) {
      contextCreateRoom(newRoomName);
      setNewRoomName("");
      setShowCreateRoom(false);
    }
  };

  return (
    <>
      {/* Toggle Button */}
      <button
        onClick={() => (isSidebarOpen ? closeSidebar() : openSidebar())}
        className="fixed bottom-4 left-4 z-50 p-2 bg-indigo-600 text-white rounded-lg shadow-lg hover:bg-indigo-700 transition md:hidden"
        title={isSidebarOpen ? "Close sidebar" : "Open sidebar"}
      >
        <PanelLeft
          className={`w-5 h-5 transition-transform ${isSidebarOpen ? "rotate-0" : "rotate-180"}`}
        />
      </button>

      {/* Sidebar */}
      <div
        className={`w-80 bg-white border-r border-gray-200 flex flex-col transition-transform duration-300 ${
          isSidebarOpen ? "translate-x-0" : "-translate-x-full"
        } fixed md:relative h-full z-40`}
      >
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

      {/* Overlay for mobile */}
      {isSidebarOpen && (
        <div
          role="button"
          tabIndex={0}
          aria-label="Close sidebar overlay"
          className="fixed inset-0 bg-black bg-opacity-50 z-30 md:hidden"
          onClick={closeSidebar}
          onKeyDown={(e) => {
            if (e.key === "Escape") {
              closeSidebar();
            }
          }}
        />
      )}
    </>
  );
}
