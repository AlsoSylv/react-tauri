import { invoke } from '@tauri-apps/api';

import { RunesRequestResponse, State } from 'interfaces';

async function getRunes(state: State): Promise<RunesRequestResponse> {
  try {
    const runes: Array<Array<string>> = await invoke('rune_names', {
      name: state.champion,
      role: state.role,
      rank: state.rank,
      region: state.region,
    });

    return { runes, completedSuccessfully: true };
  } catch (exception) {
    console.error('Got an error while trying to fetch the runes for state %o: %o', state, exception);
    return { message: 'No Data Exists!', completedSuccessfully: false };
  }
}

async function getChampionNames() {
  const championNames: string[] = await invoke('champion_names');

  return championNames.map((name) => name.replace(/['"]+/g, ''));
}

export { getRunes, getChampionNames };
