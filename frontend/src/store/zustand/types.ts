export interface GameStoreData {
  isJoinGameLoading: boolean;
  isCreateGameLoading: boolean;

  currentGameInfo: GameInfo | null;
}

export interface GameInfo {
  room_key: string;
}

export interface GameStoreActions {
  joinGame: (gameCode: string) => Promise<GameInfo | undefined>;
  createGame: () => Promise<GameInfo | undefined>;
  fetchGame: (gameCode: string) => Promise<GameInfo | undefined>;

  reset: () => void;
}

export type GameStore = GameStoreData & GameStoreActions;
