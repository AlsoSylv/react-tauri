import { useState, useEffect } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import { AutoCompleteOptions } from 'interfaces';
import { getChampionNames } from 'utils/utils';

function ChampionOptions() {
  const [champions, setChampions] = useState<AutoCompleteOptions[]>([]);
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

  const changeSelectedChampion = (_: unknown, value: AutoCompleteOptions | null) => {
    const newChampionSelection = value?.value || '';

    dispatch({ type: Actions.UPDATE_CHAMPION, payload: newChampionSelection });
  };

  return (
    <Box>
      <Autocomplete<AutoCompleteOptions>
        disablePortal
        value={champions.find(({ value }) => value === champion) || null}
        onChange={changeSelectedChampion}
        loading={isLoading}
        id="champions-select"
        options={champions}
        getOptionLabel={({ label }) => label}
        renderInput={(params) => <TextField {...params} label="Select a champion" />}
      />
    </Box>
  );
}

export default ChampionOptions;
