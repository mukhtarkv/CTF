"use client";

import { useGameStore } from "@/store/zustand/module";
import Button from "@/ui/Button";
import { useRouter } from "next/navigation";

const Home = () => {
  const createGame = useGameStore((state) => state.createGame);
  const router = useRouter();

  const makeNewGame = async () => {
    const gameInfo = await createGame();
    // TODO: toast no game info
    if (!gameInfo) return;

    router.push(`/host/${gameInfo.room_key}`);
  };

  return (
    <div className="flex w-full h-screen flex-col justify-center items-center gap-[12px]">
      <span className="text-xl font-bold">Home</span>
      <div className="flex gap-[4px]">
        <Button onClick={() => makeNewGame()}>New game</Button>
        <Button onClick={() => router.push("/join")}>Join game</Button>
      </div>
    </div>
  );
};

export default Home;
