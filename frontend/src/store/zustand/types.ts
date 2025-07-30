export interface GameStoreData {
  isJoinGameLoading: boolean;
  isCreateGameLoading: boolean;

  currentGameInfo: GameInfo | null;
}

export interface GameInfo {
  key: string;
  id: string;
}

export interface GameStoreActions {
  joinGame: (gameCode: string) => Promise<GameInfo>;
  createGame: () => Promise<GameInfo>;
  fetchGame: (gameCode: string) => Promise<GameInfo>;

  reset: () => void;
}

export type GameStore = GameStoreData & GameStoreActions;
