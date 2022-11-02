import { useState, useEffect } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';
import { useNavigate } from 'react-router-dom';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import { ChampionOptions as IChampionOptions } from 'interfaces';
import { getChampionNames } from 'utils/utils';

function ChampionOptions() {
  const [champions, setChampions] = useState<IChampionOptions[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const {
    state: { champion },
    dispatch,
  } = useGlobalContext();
  const navigate = useNavigate();

  useEffect(() => {
    const getChampions = async () => {
      const newNames = await getChampionNames();

      setChampions(newNames);
      setIsLoading(false);
    };

    getChampions();
  }, []);

  const changeSelectedChampion = (_: unknown, value: IChampionOptions | null) => {
    const newChampionSelection = value?.value || '';

    dispatch({ type: Actions.UPDATE_CHAMPION, payload: newChampionSelection });
    if (newChampionSelection) {
      console.log(newChampionSelection);

      navigate(`/champions/${newChampionSelection}`);
    }
  };

  return (
    <Box>
      <Autocomplete<IChampionOptions>
        disablePortal
        value={champions.find(({ value }) => value === champion) || null}
        onChange={changeSelectedChampion}
        loading={isLoading}
        id="champions-select"
        options={champions}
        renderInput={(params) => <TextField {...params} label="Select a champion" />}
        renderOption={(props, option) => (
          <Box component="li" sx={{ '& > img': { mr: 2, flexShrink: 0 } }} {...props}>
            <img loading="lazy" width="20" src={option.url} srcSet={`${option.url} 2x`} alt="" />
            {option.label}
          </Box>
        )}
      />
    </Box>
  );
}

export default ChampionOptions;
