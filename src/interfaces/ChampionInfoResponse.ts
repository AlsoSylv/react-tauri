import { CompleteChampionInfo } from './ChampionInfo';

type ChampionInfoResponse =
  | ({
      completedSuccessfully: true;
    } & CompleteChampionInfo)
  | {
      message: string;
      completedSuccessfully: false;
    };

export default ChampionInfoResponse;
