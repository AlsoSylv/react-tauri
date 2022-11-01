import { SyntheticEvent } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import type { AutoCompleteOption } from 'interfaces';

const roles: AutoCompleteOption[] = [
  { label: 'Default', value: 'default' },
  { label: 'Top', value: 'top' },
  { label: 'Jungle', value: 'jungle' },
  { label: 'Mid', value: 'mid' },
  { label: 'ADC', value: 'adc' },
  { label: 'Support', value: 'support' },
];

function RoleMenu() {
  const {
    state: { role },
    dispatch,
  } = useGlobalContext();

  const handleChangeRank = (_: SyntheticEvent<Element, Event>, value: AutoCompleteOption | null) => {
    const newValue = value?.value || '';

    dispatch({ type: Actions.UPDATE_ROLE, payload: newValue });
  };

  return (
    <Box>
      <Autocomplete
        disablePortal
        id="roles-select"
        defaultValue={roles[0]}
        value={roles.find(({ value }) => value === role)}
        options={roles}
        onChange={handleChangeRank}
        disableClearable
        isOptionEqualToValue={(option, value) => option.value === value.value}
        renderInput={(params) => <TextField {...params} label="Select a role" />}
      />
    </Box>
  );
}

export default RoleMenu;
