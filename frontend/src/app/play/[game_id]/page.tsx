"use client";

import Play from "@/components/Play";
import { isNil } from "lodash";
import { useParams } from "next/navigation";
import { isArray } from "util";

const PlayPage = () => {
  const params = useParams();
  const gameId = isArray(params.game_id)
    ? params.game_id.at(0)
    : params.game_id;

  // NOTE: better no data state
  if (isNil(gameId)) return <div>No game found</div>;

  return <Play gameId={gameId} />;
};

export default PlayPage;
