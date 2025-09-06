"use client";

import { useEffect, useRef } from "react";

interface Player {
  x: number;
  y: number;
  team: number;
}

interface GameCanvasProps {
  players: Player[];
  walls: Array<{ x: number; y: number }>;
  flags: Array<{ x: number; y: number; team: number; captured: boolean }>;
  flagCaptors?: [number | null, number | null];
  scores: [number, number];
  width: number;
  height: number;
}

const GameCanvas: React.FC<GameCanvasProps> = ({
  players,
  walls,
  flags,
  flagCaptors,
  scores,
  width,
  height,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const CELL_SIZE = 20;
  
  // Provide default values for flagCaptors
  const safeFlagCaptors = flagCaptors || [null, null];

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = "#f0f0f0";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw grid
    ctx.strokeStyle = "#e0e0e0";
    ctx.lineWidth = 1;
    for (let x = 0; x <= width * CELL_SIZE; x += CELL_SIZE) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, height * CELL_SIZE);
      ctx.stroke();
    }
    for (let y = 0; y <= height * CELL_SIZE; y += CELL_SIZE) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width * CELL_SIZE, y);
      ctx.stroke();
    }

    // Draw team territories (left half blue, right half red)
    const halfWidth = (width * CELL_SIZE) / 2;
    
    // Blue team territory (left)
    ctx.fillStyle = "rgba(59, 130, 246, 0.1)";
    ctx.fillRect(0, 0, halfWidth, height * CELL_SIZE);
    
    // Red team territory (right)
    ctx.fillStyle = "rgba(239, 68, 68, 0.1)";
    ctx.fillRect(halfWidth, 0, halfWidth, height * CELL_SIZE);

    // Draw center line
    ctx.strokeStyle = "#666";
    ctx.lineWidth = 2;
    ctx.setLineDash([5, 5]);
    ctx.beginPath();
    ctx.moveTo(halfWidth, 0);
    ctx.lineTo(halfWidth, height * CELL_SIZE);
    ctx.stroke();
    ctx.setLineDash([]);

    // Draw walls
    ctx.fillStyle = "#333";
    walls.forEach(wall => {
      ctx.fillRect(
        wall.x * CELL_SIZE,
        wall.y * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE
      );
    });

    // Draw flags
    flags.forEach((flag, flagIndex) => {
      const isCaptured = safeFlagCaptors[flagIndex] !== null;
      if (!isCaptured) {
        ctx.fillStyle = flag.team === 0 ? "#3b82f6" : "#ef4444";
        ctx.fillRect(
          flag.x * CELL_SIZE + 2,
          flag.y * CELL_SIZE + 2,
          CELL_SIZE - 4,
          CELL_SIZE - 4
        );
        
        // Flag pole
        ctx.fillStyle = "#8b5cf6";
        ctx.fillRect(
          flag.x * CELL_SIZE + CELL_SIZE/2 - 1,
          flag.y * CELL_SIZE,
          2,
          CELL_SIZE
        );
      } else {
        // Draw captured flag indicator at flag spawn location
        ctx.fillStyle = "rgba(128, 128, 128, 0.5)";
        ctx.fillRect(
          flag.x * CELL_SIZE + 2,
          flag.y * CELL_SIZE + 2,
          CELL_SIZE - 4,
          CELL_SIZE - 4
        );
        
        // Draw "CAPTURED" text
        ctx.fillStyle = "#666";
        ctx.font = "bold 8px Arial";
        ctx.textAlign = "center";
        ctx.fillText(
          "CAPTURED",
          flag.x * CELL_SIZE + CELL_SIZE/2,
          flag.y * CELL_SIZE + CELL_SIZE/2
        );
      }
    });

    // Draw players
    players.forEach((player, index) => {
      const playerX = player.x * CELL_SIZE;
      const playerY = player.y * CELL_SIZE;
      
      // Player circle
      ctx.fillStyle = player.team === 0 ? "#1d4ed8" : "#dc2626";
      ctx.beginPath();
      ctx.arc(
        playerX + CELL_SIZE/2,
        playerY + CELL_SIZE/2,
        CELL_SIZE/3,
        0,
        2 * Math.PI
      );
      ctx.fill();

      // Player number
      ctx.fillStyle = "white";
      ctx.font = "bold 12px Arial";
      ctx.textAlign = "center";
      ctx.fillText(
        (index + 1).toString(),
        playerX + CELL_SIZE/2,
        playerY + CELL_SIZE/2 + 4
      );
      
      // Draw flag indicator if player is carrying a flag
      for (let flagIndex = 0; flagIndex < safeFlagCaptors.length; flagIndex++) {
        if (safeFlagCaptors[flagIndex] === index) {
          // Draw small flag icon above player
          ctx.fillStyle = flagIndex === 0 ? "#3b82f6" : "#ef4444";
          ctx.fillRect(
            playerX + CELL_SIZE/2 - 3,
            playerY - 8,
            6,
            6
          );
          ctx.fillStyle = "#8b5cf6";
          ctx.fillRect(
            playerX + CELL_SIZE/2 - 1,
            playerY - 8,
            2,
            8
          );
        }
      }
    });
  }, [players, walls, flags, safeFlagCaptors, width, height]);

  return (
    <div className="flex flex-col items-center gap-4">
      <canvas
        ref={canvasRef}
        width={width * CELL_SIZE}
        height={height * CELL_SIZE}
        className="border-2 border-gray-400 bg-white"
      />
      <div className="text-sm text-gray-600 max-w-2xl text-center">
        <p><strong>Blue Team (Left):</strong> Players 1, 3 | <strong>Red Team (Right):</strong> Players 2, 4</p>
        <p>Capture the enemy flag and bring it back to your territory to score!</p>
      </div>
    </div>
  );
};

export default GameCanvas;