import config from "@/config";

const New = () => {
  return (
    <div className="flex w-full h-screen flex-col justify-center items-center gap-[32px]">
      <span className="text-xl font-bold">Game Lobby</span>
      <span className="text-lg text-gray-700 max-w-[200px] text-center leading-[24px]">
        Visit <b>{config.displayUrl}</b> and join using the code
      </span>
      <div className="bg-gray-300 px-[16px] py-[12px] rounded-lg text-3xl tracking-[12px]">
        123456
      </div>
    </div>
  );
};

export default New;
