"use client";

import { useGameStore } from "@/store/zustand/module";
import { isNil } from "lodash";
import { useEffect } from "react";

interface Props {
  gameId: string;
}

const Play = ({ gameId }: Props) => {
  const { currentGameInfo, fetchGame } = useGameStore();

  useEffect(() => {
    // TODO: fetching should be done on id not key
    const key = currentGameInfo?.key;
    if (!isNil(key)) fetchGame(key);
  }, [currentGameInfo?.key]);

  const gameKey = currentGameInfo?.key;

  return (
    <div className="flex w-full h-screen flex-col justify-center items-center gap-[32px]">
      <span className="text-xl font-bold">Joined Game</span>
      <span className="text-lg text-gray-700 max-w-[200px] text-center leading-[24px]">
        Waiting for host to start the game
      </span>
      <div className="bg-gray-300 px-[16px] py-[12px] rounded-lg text-3xl text-right flex gap-[12px]">
        {gameKey &&
          // NOTE: fancy way of putting gaps between chars
          [...gameKey.toString()].map((char, i) => <span key={i}>{char}</span>)}
      </div>
    </div>
  );
};

export default Play;
