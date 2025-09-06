"use client";

import config from "@/config";
import { useWebSocket } from "@/hooks/useWebSocket";
import { useGameStore } from "@/store/zustand/module";
import Button from "@/ui/Button";
import { useEffect, useState } from "react";
import GameCanvas from "./GameCanvas";

interface Props {
  gameId: string;
}

interface GameState {
  players: Array<{ x: number; y: number; team: number }>;
  walls: Array<{ x: number; y: number }>;
  flags: Array<{ x: number; y: number; team: number; captured: boolean }>;
  scores: [number, number];
  isStarted: boolean;
  connectedPlayers: number;
}

const Host = ({ gameId }: Props) => {
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
    connectedPlayers: 0,
  });

  const gameKey = currentGameInfo?.room_key;
  const wsUrl = gameKey ? `ws://localhost:8000/rooms/${gameKey}?role=host` : "";

  const { isConnected, sendMessage, connect, connectionState } = useWebSocket({
    url: wsUrl,
    onMessage: (message) => {
      console.log("Host received:", message);
      
      switch (message.type) {
        case "welcome":
          console.log("Host connected to room:", message.room);
          break;
        case "user_joined":
          setGameState(prev => ({
            ...prev,
            connectedPlayers: Math.min(4, prev.connectedPlayers + 1)
          }));
          break;
        case "user_left":
          setGameState(prev => ({
            ...prev,
            connectedPlayers: Math.max(0, prev.connectedPlayers - 1)
          }));
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
      }
    },
    onOpen: () => {
      console.log("Host WebSocket connected");
    },
    onClose: () => {
      console.log("Host WebSocket disconnected");
    }
  });

  useEffect(() => {
    if (gameId) {
      fetchGame(gameId);
    }
  }, [gameId, fetchGame]);

  useEffect(() => {
    if (gameKey && wsUrl && !isConnected && connectionState === 'disconnected') {
      console.log('Attempting to connect with gameKey:', gameKey);
      connect();
    }
  }, [gameKey, wsUrl, isConnected, connectionState, connect]);

  const startGame = () => {
    if (isConnected) {
      sendMessage({ type: "start_game" });
    }
  };

  return (
    <div className="flex w-full min-h-screen flex-col items-center gap-8 p-4">
      <div className="text-center">
        <h1 className="text-2xl font-bold mb-2">Capture The Flag - Host</h1>
        <p className="text-gray-600">
          Connection: {isConnected ? "✅ Connected" : "❌ Disconnected"}
        </p>
      </div>

      {!gameState.isStarted ? (
        <div className="flex flex-col items-center gap-8 max-w-md mx-auto">
          <div className="text-center">
            <p className="text-lg text-gray-700 mb-4">
              Visit <b>{config.displayUrl}</b> and join using the code:
            </p>
            <div className="flex flex-col items-center gap-4">
              <div className="bg-gray-300 px-6 py-4 rounded-lg text-3xl font-mono select-all">
                {gameKey}
              </div>
              <Button
                size="sm"
                variant="secondary"
                onClick={() => {
                  if (gameKey) {
                    navigator.clipboard.writeText(gameKey);
                    // You could add a toast notification here
                  }
                }}
              >
                📋 Copy Code
              </Button>
            </div>
          </div>

          <div className="text-center w-full">
            <p className="text-lg mb-4">
              Players connected: <span className="font-bold">{gameState.connectedPlayers}/4</span>
            </p>
            <div className="flex justify-center">
              <Button 
                onClick={startGame} 
                isDisabled={!isConnected || gameState.connectedPlayers === 0}
                variant="primary"
                size="lg"
                className="px-8 py-3"
              >
                Start Game
              </Button>
            </div>
          </div>
        </div>
      ) : (
        <div className="w-full max-w-4xl">
          <div className="flex justify-between w-full px-4 mb-4">
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

export default Host;
