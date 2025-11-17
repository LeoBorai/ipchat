import { useState } from "react";
import { Wifi } from "lucide-react";

interface SignInFormProps {
  onSubmit: (username: string) => void;
  initialServerUrl?: string;
}

export function SignInForm({ onSubmit }: SignInFormProps) {
  const [username, setUsername] = useState("");

  const handleSubmit = () => {
    if (username.trim()) {
      onSubmit(username);
    }
  };

  return (
    <div className="flex items-center justify-center min-h-screen from-blue-50 to-indigo-100">
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
              Username
            </label>
            <input
              type="text"
              placeholder="Enter your username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              onKeyPress={(e) =>
                e.key === "Enter" && username.trim() && handleSubmit()
              }
              className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
            />
          </div>

          <button
            onClick={handleSubmit}
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
