import { SyntheticEvent } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import type { AutoCompleteOption } from 'interfaces';

function RoleMenu() {
  const {
    state: { role, roleList },
    dispatch,
  } = useGlobalContext();

  const handleChangeRank = (_: SyntheticEvent<Element, Event>, value: AutoCompleteOption<string> | null) => {
    const newValue = value?.value || '';

    dispatch({ type: Actions.UPDATE_ROLE, payload: newValue });
  };

  return (
    <Box>
      <Autocomplete
        disablePortal
        id="roles-select"
        defaultValue={roleList[0]}
        value={roleList.find(({ value }) => value === role)}
        options={roleList}
        onChange={handleChangeRank}
        disableClearable
        isOptionEqualToValue={(option, value) => option?.value === value?.value}
        renderInput={(params) => <TextField {...params} label="Select a role" />}
      />
    </Box>
  );
}

export default RoleMenu;
