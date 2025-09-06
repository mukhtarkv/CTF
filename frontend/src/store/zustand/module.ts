import axios from "axios";
import { create } from "zustand";
import { devtools } from "zustand/middleware";

import config from "../../config";
import { GameInfo, GameStore, GameStoreData } from "./types";

const initialState: GameStoreData = {
  isJoinGameLoading: false,
  isCreateGameLoading: false,

  currentGameInfo: null,
};

export const useGameStore = create<GameStore>()(
  devtools(
    (set, get) =>
      ({
        ...initialState,

        createGame: async () => {
          try {
            const response = await axios.post(`${config.apiBaseUrl}/rooms`);

            const result: GameInfo = response.data;
            set({ currentGameInfo: result });
            return result;
          } catch (error) {
            if (axios.isAxiosError(error)) {
              console.error(error);
            }
          }
        },

        joinGame: async (key: string) => {
          // For joining, we just need to set the room key and connect via WebSocket
          const gameInfo: GameInfo = { room_key: key };
          set({ currentGameInfo: gameInfo });
          return gameInfo;
        },

        fetchGame: async (key: string) => {
          // For fetching, we just need to set the room key
          const gameInfo: GameInfo = { room_key: key };
          set({ currentGameInfo: gameInfo });
          return gameInfo;
        },

        reset: () => set(initialState),
      }) as GameStore,
    { name: "game-storage" },
  ),
);
