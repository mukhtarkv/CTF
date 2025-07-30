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

        fetchGame: async (key: string) => {
          try {
            const response = await axios.get(
              `${config.apiBaseUrl}/rooms/${key}/join`,
            );

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
          try {
            const response = await axios.get(
              `${config.apiBaseUrl}/rooms/${key}/join`,
            );

            const result: GameInfo = response.data;
            set({ currentGameInfo: result });
            return result;
          } catch (error) {
            if (axios.isAxiosError(error)) {
              console.error(error);
            }
          }
        },

        reset: () => set(initialState),
      }) as GameStore,
    { name: "game-storage" },
  ),
);
