const roomKey = "123456";
const ws = new WebSocket(`ws://localhost:8000/ws/rooms/${roomKey}?role=player`);

ws.addEventListener("open", () => {
  console.log("connected to room", roomKey);
});

ws.addEventListener("message", ev => {
  console.log("received:", ev.data);
});

// send a simple text/event
ws.send(JSON.stringify({ type: "chat", content: "hello everyone" }));
