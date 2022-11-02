import { invoke } from '@tauri-apps/api';

import { ChampionInfoResponse, State, RuneTrees, Shards, ChampionOptions, ChampionInfo } from 'interfaces';
import ValidatedStateResponse from 'interfaces/ValidatedStateResponse';

async function getChampionInfo(state: State): Promise<ChampionInfoResponse> {
  try {
    const requestArgs = {
      name: state.champion,
      role: state.role,
      rank: state.rank,
      region: state.region,
    };

    const [runes, shards, championInfo] = await Promise.all([
      invoke<RuneTrees>('rune_names', requestArgs),
      invoke<Shards>('shard_names', requestArgs),
      invoke<ChampionInfo>('champion_info', requestArgs),
    ]);

    return { ...championInfo, runes, shards, completedSuccessfully: true };
  } catch (exception) {
    console.error('Got an error while trying to fetch the runes for state %o', state);
    console.error(exception);
    return { message: 'No Data Exists!', completedSuccessfully: false };
  }
}

function getChampionNames() {
  return invoke<ChampionOptions[]>('champion_names');
}

const validateState = (state: State): ValidatedStateResponse => {
  const { champion, role } = state;

  if (champion === '') {
    return { isValid: false, message: 'Please Enter A Champion Name' };
  }

  if (role !== 'default' && role === '') {
    return { isValid: false, message: 'Please Select a Role' };
  }

  return { isValid: true, message: '' };
};

const getRandomKey = () => crypto.randomUUID();

const createArrayFromLength = (length: number) => [...Array(length)].map(getRandomKey);

export { getChampionInfo, getChampionNames, validateState, createArrayFromLength, getRandomKey };
