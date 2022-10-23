import ChampionInfo from './ChampionInfo';

type ChampionInfoResponse =
  | ({
      completedSuccessfully: true;
    } & ChampionInfo)
  | {
      message: string;
      completedSuccessfully: false;
    };

export default ChampionInfoResponse;
