import { SyntheticEvent } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import AutoCompleteOption from 'interfaces/AutoCompleteOption';

const regions: AutoCompleteOption[] = [
  { label: 'World', value: 'world' },
  { label: 'North America', value: 'na1' },
  { label: 'EU West', value: 'euw1' },
  { label: 'Korea', value: 'kr' },
  { label: 'Brazil', value: 'br1' },
  { label: 'EU North', value: 'eun1' },
  { label: 'Japan', value: 'jp1' },
  { label: 'LA North', value: 'la1' },
  { label: 'LA South', value: 'la2' },
  { label: 'OCE', value: 'oc1' },
  { label: 'Russia', value: 'ru' },
  { label: 'Turkey', value: 'tr1' },
];

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
