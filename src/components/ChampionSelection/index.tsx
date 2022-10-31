import { useState, useEffect } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import { getChampionNames } from 'utils/utils';

function ChampionOptions() {
  const [champions, setChampions] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const {
    state: { champion },
    dispatch,
  } = useGlobalContext();

  useEffect(() => {
    const getChampions = async () => {
      const newNames = await getChampionNames();

      setChampions(newNames);
      setIsLoading(false);
    };

    getChampions();
  }, []);

  const changeSelectedChampion = (_: unknown, value: string | null) => {
    const newChampionSelection = value || '';

    dispatch({ type: Actions.UPDATE_CHAMPION, payload: newChampionSelection });
  };

  return (
    <Box>
      <Autocomplete<string>
        disablePortal
        value={champion || null}
        onChange={changeSelectedChampion}
        loading={isLoading}
        id="champions-select"
        options={champions}
        renderInput={(params) => <TextField {...params} label="Select a champion" />}
      />
    </Box>
  );
}

export default ChampionOptions;
