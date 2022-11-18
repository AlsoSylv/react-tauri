import { SyntheticEvent } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';
import { invoke } from '@tauri-apps/api';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import { AutoCompleteOption } from 'interfaces/AutoCompleteOption';

const regions: AutoCompleteOption[] = [];

invoke<string[]>('regions').then((region) => {
  region.map((Region) => regions.push({ label: Region, value: Region }));
});

function RegionMenu() {
  const {
    state: { region },
    dispatch,
  } = useGlobalContext();

  const handleChangeRank = (_: SyntheticEvent<Element, Event>, value: AutoCompleteOption | null) => {
    const newValue = value?.value || '';

    dispatch({ type: Actions.UPDATE_REGION, payload: newValue });
  };

  return (
    <Box>
      <Autocomplete
        disablePortal
        id="region-select"
        value={regions.find(({ value }) => value === region)}
        isOptionEqualToValue={(option, value) => option.value === value.value}
        disableClearable
        options={regions}
        onChange={handleChangeRank}
        renderInput={(params) => <TextField {...params} label="Select a region" />}
      />
    </Box>
  );
}

export default RegionMenu;
