export interface GameStoreData {
  isJoinGameLoading: boolean;
  isCreateGameLoading: boolean;
}

export interface GameStoreActions {
  joinGame: (gameCode: string) => Promise<void>;
  createGame: () => Promise<void>;

  reset: () => void;
}

export type GameStore = GameStoreData & GameStoreActions;
