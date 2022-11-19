import ChampionBuild from './ChampionBuild';
import ChampionInfo from './ChampionInfo';

type ChampionData = ChampionBuild & ChampionInfo;

interface SuccessfulChampionInfoResponse extends ChampionData {
  completedSuccessfully: true;
}

interface FailedChampionInfoResponse {
  message: string;
  completedSuccessfully: false;
}

type ChampionInfoResponse = SuccessfulChampionInfoResponse | FailedChampionInfoResponse;

export { ChampionData, ChampionInfoResponse };
