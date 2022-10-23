import { invoke } from '@tauri-apps/api';

import { ChampionInfoResponse, State } from 'interfaces';
import ValidatedStateResponse from 'interfaces/ValidatedStateResponse';

async function getChampionInfo(state: State): Promise<ChampionInfoResponse> {
  try {
    const requestArgs = {
      name: state.champion,
      role: state.role,
      rank: state.rank,
      region: state.region,
    };

    const [runes, shards, winRate] = await Promise.all([
      invoke<[string[][], string[][], string[][]]>('rune_names', requestArgs),
      invoke<string[]>('shard_names', requestArgs),
      invoke<string>('win_rate', requestArgs),
    ]);

    return { runes, shards, winRate, completedSuccessfully: true };
  } catch (exception) {
    console.error('Got an error while trying to fetch the runes for state %o: %o', state, exception);
    return { message: 'No Data Exists!', completedSuccessfully: false };
  }
}

async function getChampionNames() {
  const championNames: string[] = await invoke('champion_names');

  return championNames.map((name) => name.replace(/['"]+/g, ''));
}

const validateState = (state: State): ValidatedStateResponse => {
  const { champion, role } = state;

  if (champion === '' && role === 'none') {
    return { isValid: false, message: 'Please Enter A Champion Name And Select A Role' };
  }

  if (champion === '') {
    return { isValid: false, message: 'Please Enter A Champion Name' };
  }

  if (role === 'none' || role === '') {
    return { isValid: false, message: 'Please Select a Role' };
  }

  return { isValid: true, message: '' };
};

export { getChampionInfo, getChampionNames, validateState };
