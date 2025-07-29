import Button from "@/ui/Button";
import Input from "@/ui/Input";

const Join = () => {
  return (
    <div className="flex w-full h-screen flex-col justify-center items-center gap-[12px]">
      <span className="text-xl font-bold">Join Game</span>
      <div className="flex flex-col gap-[8px] w-full max-w-[240px]">
        <Input placeholder="Name" />
        <Input placeholder="Game code" />
        <Button>Join game</Button>
      </div>
    </div>
  );
};

export default Join;
