import { SyntheticEvent } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import { AutoCompleteOption } from 'interfaces';

function RegionMenu() {
  const {
    state: { region, regionList },
    dispatch,
  } = useGlobalContext();

  const handleChangeRank = (_: SyntheticEvent<Element, Event>, value: AutoCompleteOption<string> | null) => {
    const newValue = value?.value || '';

    dispatch({ type: Actions.UPDATE_REGION, payload: newValue });
  };

  return (
    <Box>
      <Autocomplete
        disablePortal
        id="region-select"
        value={regionList.find(({ value }) => value === region)}
        isOptionEqualToValue={(option, value) => option.value === value.value}
        disableClearable
        options={regionList}
        onChange={handleChangeRank}
        renderInput={(params) => <TextField {...params} label="Select a region" />}
      />
    </Box>
  );
}

export default RegionMenu;
