"use client";

import Button from "@/ui/Button";
import { useRouter } from "next/navigation";

const Home = () => {
  const router = useRouter();

  return (
    <div className="flex w-full h-screen flex-col justify-center items-center gap-[12px]">
      <span className="text-xl font-bold">Home</span>
      <div className="flex gap-[4px]">
        <Button onClick={() => router.push("/new")}>New game</Button>
        <Button onClick={() => router.push("/join")}>Join game</Button>
      </div>
    </div>
  );
};

export default Home;
