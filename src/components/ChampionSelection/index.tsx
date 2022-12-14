import { useState, useEffect } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import { ChampionOptions as IChampionOptions } from 'interfaces';
import { getChampionNames } from 'utils/utils';

function ChampionOptions() {
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const {
    state: { champion, selectedLanguage, championList },
    dispatch,
  } = useGlobalContext();

  useEffect(() => {
    const getChampions = async () => {
      if (selectedLanguage) {
        const newNames = await getChampionNames(selectedLanguage);

        dispatch({ type: Actions.SET_CHAMPIONS_LIST, payload: newNames });
        setIsLoading(false);
      }
    };

    getChampions();
  }, [selectedLanguage, dispatch]);

  const changeSelectedChampion = (_: unknown, value: IChampionOptions | null) => {
    const newChampionSelection = value ? { value: value.value, label: value.label } : null;

    dispatch({ type: Actions.UPDATE_CHAMPION, payload: newChampionSelection });
  };

  return (
    <Box>
      <Autocomplete<IChampionOptions>
        disablePortal
        value={championList.find(({ value }) => value.id === champion?.value.id) || null}
        onChange={changeSelectedChampion}
        loading={isLoading}
        id="champions-select"
        options={championList}
        renderInput={(params) => <TextField {...params} label="Select a champion" />}
        renderOption={(props, option) => (
          <Box component="li" sx={{ '& > img': { mr: 2, flexShrink: 0 } }} {...props}>
            <img
              loading="lazy"
              width="20"
              src={`../champions${option.localImage}`}
              srcSet={`../champions${option.localImage} 2x`}
              alt={option.label}
              onError={({ currentTarget }) => {
                currentTarget.onerror = null;
                currentTarget.src = option.url;
                currentTarget.srcset = `${option.url} 2x`;
              }}
            />
            {option.label}
          </Box>
        )}
      />
    </Box>
  );
}

export default ChampionOptions;
