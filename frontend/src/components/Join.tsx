"use client";

import { useGameStore } from "@/store/zustand/module";
import Button from "@/ui/Button";
import Input from "@/ui/Input";
import { isEmpty } from "lodash";
import { useRouter } from "next/navigation";
import { useState } from "react";

const Join = () => {
  const joinGame = useGameStore((state) => state.joinGame);
  const router = useRouter();
  const [key, setKey] = useState("");
  const [name, setName] = useState("");

  const onJoinGame = async () => {
    const gameInfo = await joinGame(key);
    // TODO: toast no game info
    if (!gameInfo) return;
    router.push(`/play/${gameInfo.room_key}`);
  };

  const isDisabled = isEmpty(key) || isEmpty(name);

  return (
    <div className="flex w-full h-screen flex-col justify-center items-center gap-[12px]">
      <span className="text-xl font-bold">Join Game</span>
      <div className="flex flex-col gap-[8px] w-full max-w-[240px]">
        <Input
          placeholder="Name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <Input
          placeholder="Game code"
          value={key}
          onChange={(e) => setKey(e.target.value)}
        />
        <Button onClick={onJoinGame} isDisabled={isDisabled}>
          Join game
        </Button>
      </div>
    </div>
  );
};

export default Join;
