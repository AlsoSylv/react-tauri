import { SyntheticEvent } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';
import { invoke } from '@tauri-apps/api';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import type { AutoCompleteOption } from 'interfaces';

const roles: AutoCompleteOption[] = [];

invoke<string[]>('roles').then((role) => {
  role.map((Role) => roles.push({ label: Role, value: Role }));
});

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
