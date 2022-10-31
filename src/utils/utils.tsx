import type { ReactNode } from 'react';

import { Unstable_Grid2 as Grid, Avatar } from '@mui/material';
import { invoke } from '@tauri-apps/api';

import { ChampionInfoResponse, State, Trees, Shards, RuneData, Shard } from 'interfaces';
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
      invoke<Trees>('rune_names', requestArgs),
      invoke<Shards>('shard_names', requestArgs),
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

const runeMap = ({ name, image, active }: RuneData | Shard): ReactNode => (
  <Grid key={name} sm sx={{ display: 'flex', alignSelf: 'center', justifyContent: 'center' }}>
    <Avatar src={image} alt={name} sx={{ ...(!active && { filter: 'grayscale(100%)', opacity: '.3' }) }} />
  </Grid>
);

export { getChampionInfo, getChampionNames, validateState, runeMap };
