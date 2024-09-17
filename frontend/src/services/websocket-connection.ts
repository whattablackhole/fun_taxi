export class WebSocketService {
  private socket: WebSocket;

  constructor(base = "ws://localhost:8082") {
    this.socket = new WebSocket(`${base}/ws/default/`);

    this.socket.onopen = () => {
      console.log("WebSocket connection established");
    };

    this.socket.onmessage = (event) => {
      const data = JSON.parse(event.data);
      console.log("Message from server:", data);
    };

    this.socket.onclose = () => {
      console.log("WebSocket connection closed");
    };

    this.socket.onerror = (error) => {
      console.error("WebSocket error:", error);
    };
  }

  public disconnect() {
    if (this.socket) {
      this.socket.close();
    }
  }
}
