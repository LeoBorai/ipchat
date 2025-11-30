import { useState } from "react";

import { SignInForm } from "../components/atoms/SignInForm";
import { Sidebar } from "../components/molecules/Sidebar";
import { ChatArea } from "../components/organisms/ChatArea";
import { useChatWebSocket } from "../contexts/ChatWebSocketContext";
import { useNode } from "../contexts/NodeContext";

export function Home() {
  const [username, setUsername] = useState("");
  const [isSetup, setIsSetup] = useState(false);
  const { serverUrl } = useNode();
  const { connect } = useChatWebSocket();

  const handleSetup = (user: string) => {
    setUsername(user);
    setIsSetup(true);
    connect(serverUrl);
  };

  if (!isSetup) {
    return <SignInForm onSubmit={handleSetup} />;
  }

  return (
    <div className="flex h-screen bg-gray-100">
      <Sidebar username={username} />
      <ChatArea username={username} />
    </div>
  );
}
