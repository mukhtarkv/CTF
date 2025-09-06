"use client";

import { useWebSocket } from "@/hooks/useWebSocket";
import { useGameStore } from "@/store/zustand/module";
import { useEffect, useState } from "react";
import GameCanvas from "./GameCanvas";
import GameControls from "./GameControls";

interface Props {
  gameId: string;
}

interface GameState {
  players: Array<{ x: number; y: number; team: number }>;
  walls: Array<{ x: number; y: number }>;
  flags: Array<{ x: number; y: number; team: number; captured: boolean }>;
  flagCaptors: [number | null, number | null];
  scores: [number, number];
  isStarted: boolean;
  playerId: number;
  sessionId: string;
}

const Play = ({ gameId }: Props) => {
  const { currentGameInfo, fetchGame } = useGameStore();
  const [gameState, setGameState] = useState<GameState>({
    players: [],
    walls: [
      { x: 0, y: 1 }, { x: 27, y: 1 }, { x: 8, y: 2 }, { x: 19, y: 2 },
      { x: 8, y: 6 }, { x: 9, y: 6 }, { x: 18, y: 6 }, { x: 19, y: 6 },
      { x: 8, y: 7 }, { x: 9, y: 7 }, { x: 18, y: 7 }, { x: 19, y: 7 },
      { x: 8, y: 11 }, { x: 19, y: 11 }, { x: 0, y: 12 }, { x: 27, y: 12 }
    ],
    flags: [
      { x: 0, y: 7, team: 0, captured: false },
      { x: 27, y: 6, team: 1, captured: false }
    ],
    flagCaptors: [null, null],
    scores: [0, 0],
    isStarted: false,
    playerId: -1,
    sessionId: "",
  });

  const gameKey = currentGameInfo?.room_key;
  const wsUrl = gameKey ? `ws://localhost:8000/rooms/${gameKey}?role=player` : "";

  const { isConnected, sendMessage, connect, connectionState } = useWebSocket({
    url: wsUrl,
    onMessage: (message) => {
      console.log("Player received:", message);
      
      switch (message.type) {
        case "welcome":
          setGameState(prev => ({
            ...prev,
            sessionId: message.session_id
          }));
          break;
        case "error":
          alert(message.message || "An error occurred");
          break;
        case "user_joined":
          // Update player ID if this message is about our session
          setGameState(prev => {
            if (message.session_id === prev.sessionId) {
              return {
                ...prev,
                playerId: message.player_id
              };
            }
            return prev;
          });
          break;
        case "positions":
          if (message.players && Array.isArray(message.players)) {
            const players = message.players.map((pos: [number, number], index: number) => ({
              x: Math.floor(pos[0]),
              y: Math.floor(pos[1]),
              team: index % 2
            }));
            const flagCaptors = message.flag_captors || [null, null];
            const scores = message.scores || [0, 0];
            setGameState(prev => ({ 
              ...prev, 
              players, 
              flagCaptors,
              scores
            }));
          }
          break;
        case "game_started":
          setGameState(prev => ({ ...prev, isStarted: true }));
          break;
        case "chat":
          console.log(`${message.from}: ${message.content}`);
          break;
      }
    },
    onOpen: () => {
      console.log("Player WebSocket connected");
    },
    onClose: () => {
      console.log("Player WebSocket disconnected");
    }
  });

  useEffect(() => {
    if (gameId) {
      fetchGame(gameId);
    }
  }, [gameId, fetchGame]);

  useEffect(() => {
    if (gameKey && wsUrl && !isConnected && connectionState === 'disconnected') {
      console.log('Player attempting to connect with gameKey:', gameKey);
      connect();
    }
  }, [gameKey, wsUrl, isConnected, connectionState, connect]);

  const handleMove = (dx: number, dy: number) => {
    if (isConnected && gameState.isStarted) {
      sendMessage({ type: "move", dx, dy });
    }
  };

  return (
    <div className="flex w-full min-h-screen flex-col items-center gap-8 p-4">
      <div className="text-center">
        <h1 className="text-2xl font-bold mb-2">Capture The Flag - Player</h1>
        <p className="text-gray-600">
          Connection: {isConnected ? "✅ Connected" : "❌ Disconnected"}
        </p>
        {gameState.playerId >= 0 && (
          <p className="text-sm text-gray-600">
            You are Player {gameState.playerId + 1} on {gameState.playerId % 2 === 0 ? "Blue" : "Red"} team
          </p>
        )}
      </div>

      {!gameState.isStarted ? (
        <div className="flex flex-col items-center gap-6">
          <div className="text-center">
            <p className="text-lg text-gray-700 mb-4">
              Waiting for host to start the game...
            </p>
            <div className="bg-gray-300 px-6 py-4 rounded-lg text-2xl font-mono flex gap-2">
              <span className="select-all">{gameKey}</span>
            </div>
          </div>
        </div>
      ) : (
        <div className="w-full max-w-4xl flex flex-col items-center gap-6">
          <div className="flex justify-between w-full px-4">
            <div className="text-lg font-bold text-blue-600">
              Blue Team: {gameState.scores[0]}
            </div>
            <div className="text-lg font-bold text-red-600">
              Red Team: {gameState.scores[1]}
            </div>
          </div>
          <GameCanvas
            players={gameState.players}
            walls={gameState.walls}
            flags={gameState.flags}
           flagCaptors={gameState.flagCaptors}
            scores={gameState.scores}
            width={28}
            height={14}
          />
          <GameControls
            onMove={handleMove}
            isGameStarted={gameState.isStarted}
            playerId={gameState.playerId}
          />
        </div>
      )}

      <div className="text-sm text-gray-500 text-center max-w-2xl">
        <p><strong>Game Rules:</strong></p>
        <p>• Blue team (players 1,3) vs Red team (players 2,4)</p>
        <p>• Capture the enemy flag and bring it back to your territory to score</p>
        <p>• Players are reset if caught in enemy territory</p>
      </div>
    </div>
  );
};

export default Play;
