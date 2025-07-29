import axios from "axios";
import { isEmpty, isNil, max } from "lodash";
import { create } from "zustand";
import { devtools } from "zustand/middleware";

import config from "../../config";
import { GameStoreData, GameStore } from "./types";

const initialState: GameStoreData = {
  isJoinGameLoading: false,
  isCreateGameLoading: false,
};

export const useGameStore = create<GameStore>()(
  devtools(
    (set, get) =>
      ({
        ...initialState,

        createGame: async () => {
          try {
            const response = await axios.post(
              `${config.apiBaseUrl}/rooms`,
              {},
              {
                headers: {
                  "Content-Type": "application/json",
                },
              },
            );

            //const result: string[] = response.data;
            //return result;
          } catch (error) {
            if (axios.isAxiosError(error)) {
              console.error(error);
            }
          }
        },

        joinGame: async (pin: string) => {
          try {
            const response = await axios.post(
              `${config.apiBaseUrl}/rooms`,
              {},
              {
                headers: {
                  "Content-Type": "application/json",
                },
              },
            );

            //const result: string[] = response.data;
            //return result;
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
