"use client";

import Button from "@/ui/Button";
import { useEffect, useState } from "react";

interface GameControlsProps {
  onMove: (dx: number, dy: number) => void;
  isGameStarted: boolean;
  playerId: number;
}

const GameControls = ({ onMove, isGameStarted, playerId }: GameControlsProps) => {
  const [pressedKeys, setPressedKeys] = useState<Set<string>>(new Set());

  useEffect(() => {
    if (!isGameStarted) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      e.preventDefault();
      const key = e.key.toLowerCase();
      
      if (pressedKeys.has(key)) return;
      
      setPressedKeys(prev => new Set(prev).add(key));

      // Player movement based on player ID
      let dx = 0, dy = 0;
      
      // WASD controls work for all players (universal)
      if (key === 'w') dy = -1;
      if (key === 's') dy = 1;
      if (key === 'a') dx = -1;
      if (key === 'd') dx = 1;
      
      // Arrow keys work for all players (universal)
      if (key === 'arrowup') dy = -1;
      if (key === 'arrowdown') dy = 1;
      if (key === 'arrowleft') dx = -1;
      if (key === 'arrowright') dx = 1;
      
      // Additional player-specific controls for variety
      if (playerId === 2) {
        // FGHT for player 3 (additional option)
        if (key === 'f') dy = -1;
        if (key === 'h') dy = 1;
        if (key === 'g') dx = -1;
        if (key === 't') dx = 1;
      } else if (playerId === 3) {
        // JKLI for player 4 (additional option)
        if (key === 'j') dy = -1;
        if (key === 'l') dy = 1;
        if (key === 'k') dx = -1;
        if (key === 'i') dx = 1;
      }

      if (dx !== 0 || dy !== 0) {
        onMove(dx, dy);
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      const key = e.key.toLowerCase();
      setPressedKeys(prev => {
        const newSet = new Set(prev);
        newSet.delete(key);
        return newSet;
      });
    };

    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    };
  }, [onMove, isGameStarted, playerId, pressedKeys]);

  const getControlsText = () => {
    switch (playerId) {
      case 0: return "WASD or Arrow keys to move";
      case 1: return "WASD or Arrow keys to move";
      case 2: return "WASD, Arrow keys, or F/G/H/T to move";
      case 3: return "WASD, Arrow keys, or J/K/L/I to move";
      default: return "Waiting for assignment...";
    }
  };

  const handleButtonMove = (dx: number, dy: number) => {
    if (isGameStarted) {
      onMove(dx, dy);
    }
  };

  if (!isGameStarted) {
    return (
      <div className="text-center text-gray-600">
        <p>Game not started yet. Waiting for host...</p>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center gap-4">
      <div className="text-sm font-medium text-gray-700">
        Player {playerId + 1} - {getControlsText()}
      </div>
      
      {/* Touch/Click controls for mobile */}
      <div className="flex flex-col items-center gap-2">
        <Button
          size="sm"
          onClick={() => handleButtonMove(0, -1)}
          className="w-12 h-12"
        >
          ↑
        </Button>
        <div className="flex gap-2">
          <Button
            size="sm"
            onClick={() => handleButtonMove(-1, 0)}
            className="w-12 h-12"
          >
            ←
          </Button>
          <Button
            size="sm"
            onClick={() => handleButtonMove(0, 1)}
            className="w-12 h-12"
          >
            ↓
          </Button>
          <Button
            size="sm"
            onClick={() => handleButtonMove(1, 0)}
            className="w-12 h-12"
          >
            →
          </Button>
        </div>
      </div>
    </div>
  );
};

export default GameControls;