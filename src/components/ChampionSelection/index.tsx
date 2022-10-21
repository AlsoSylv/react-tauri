import React, { useState, useEffect } from 'react';

import { Autocomplete, createFilterOptions, TextField } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import { getChampionNames } from 'utils';

const filterOptions = createFilterOptions<string>({
  limit: 5,
});

function ChampionOptions() {
  const [champions, setChampions] = useState<string[]>([]);
  const {
    state: { champion },
    setState,
  } = useGlobalContext();

  useEffect(() => {
    const getChampions = async () => {
      const newNames = await getChampionNames();
      setChampions(newNames);
    };

    getChampions();
  }, []);

  const changeSelectedChampion = (_: unknown, value: string | null) => {
    const newChampionSelection = value || '';

    setState({ type: Actions.UPDATE_CHAMPION, payload: newChampionSelection });
  };

  if (champions === null) {
    return <p>loading...</p>;
  }

  return (
    <div id="champion-popup">
      <Autocomplete<string>
        disablePortal
        value={champion}
        onChange={changeSelectedChampion}
        id="combo-box-demo"
        options={champions}
        sx={{ width: 300 }}
        filterOptions={filterOptions}
        renderInput={(params) => <TextField {...params} label="Select a champion" />}
      />
    </div>
  );
}

export default ChampionOptions;
