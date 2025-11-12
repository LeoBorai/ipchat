export type ServerMessage =
  | { type: "RoomCreated"; room: any }
  | { type: "RoomJoined"; room: any }
  | { type: "NewMessage"; message: any }
  | { type: "PeerList"; peers: any[] }
  | { type: "RoomList"; rooms: any[] }
  | { type: "Error"; message: string };

export type ConnectionStatus =
  | "Disconnected"
  | "Connecting..."
  | "Connected"
  | "Error"
  | "Reconnecting..."
  | "Connection Failed";

export interface ChatWebSocketConfig {
  serverUrl: string;
  reconnectDelay?: number;
  onConnectionChange?: (connected: boolean, status: ConnectionStatus) => void;
  onMessage?: (message: ServerMessage) => void;
  onInitialConnect?: () => void;
}

export class ChatWebSocketService {
  private ws: WebSocket | null = null;
  private reconnectTimeout: NodeJS.Timeout | null = null;
  private config: ChatWebSocketConfig;
  private shouldReconnect: boolean = false;

  constructor(config: ChatWebSocketConfig) {
    this.config = {
      reconnectDelay: 3000,
      ...config,
    };
  }

  public connect(): void {
    this.shouldReconnect = true;
    this.createConnection();
  }

  private createConnection(): void {
    try {
      this.config.onConnectionChange?.(false, "Connecting...");
      this.ws = new WebSocket(this.config.serverUrl);

      this.ws.onopen = () => {
        console.log("Connected to server");
        this.config.onConnectionChange?.(true, "Connected");
        this.config.onInitialConnect?.();
      };

      this.ws.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data) as ServerMessage;
          console.log("Received:", message);
          this.config.onMessage?.(message);
        } catch (error) {
          console.error("Failed to parse message:", error);
        }
      };

      this.ws.onerror = (error) => {
        console.error("WebSocket error:", error);
        this.config.onConnectionChange?.(false, "Error");
      };

      this.ws.onclose = () => {
        console.log("Disconnected from server");
        this.config.onConnectionChange?.(false, "Disconnected");

        if (this.shouldReconnect) {
          this.reconnectTimeout = setTimeout(() => {
            this.config.onConnectionChange?.(false, "Reconnecting...");
            this.createConnection();
          }, this.config.reconnectDelay);
        }
      };
    } catch (error) {
      console.error("Failed to connect:", error);
      this.config.onConnectionChange?.(false, "Connection Failed");
    }
  }

  public disconnect(): void {
    this.shouldReconnect = false;

    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
      this.reconnectTimeout = null;
    }

    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  public send(message: any): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    } else {
      console.error("WebSocket is not connected");
    }
  }

  public isConnected(): boolean {
    return this.ws !== null && this.ws.readyState === WebSocket.OPEN;
  }

  public updateServerUrl(serverUrl: string): void {
    this.config.serverUrl = serverUrl;
  }

  // API methods
  public listNodes(): void {
    this.send({ type: "ListNodes" });
  }

  public listRooms(): void {
    this.send({ type: "ListRooms" });
  }

  public createRoom(name: string): void {
    this.send({ type: "CreateRoom", name });
  }

  public joinRoom(roomId: string, peerIp: string): void {
    this.send({ type: "JoinRoom", room_id: roomId, peer_ip: peerIp });
  }

  public sendMessage(roomId: string, content: string, sender: string): void {
    this.send({ type: "SendMessage", room_id: roomId, content, sender });
  }
}
