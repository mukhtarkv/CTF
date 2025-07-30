"use client";

import Host from "@/components/Host";
import { isNil } from "lodash";
import { useParams } from "next/navigation";
import { isArray } from "util";

const HostPage = () => {
  const params = useParams();
  const gameId = isArray(params.game_id)
    ? params.game_id.at(0)
    : params.game_id;

  // NOTE: better no data state
  if (isNil(gameId)) return <div>No game found</div>;

  return <Host gameId={gameId} />;
};

export default HostPage;
