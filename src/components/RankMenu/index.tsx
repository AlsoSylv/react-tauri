import { SyntheticEvent } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import { AutoCompleteOption } from 'interfaces';

function RankMenu() {
  const {
    state: { rank, rankList },
    dispatch,
  } = useGlobalContext();

  const handleChangeRank = (_: SyntheticEvent<Element, Event>, value: AutoCompleteOption<string> | null) => {
    const newValue = value?.value || '';

    dispatch({ type: Actions.UPDATE_RANK, payload: newValue });
  };

  return (
    <Box>
      <Autocomplete
        disablePortal
        disableClearable
        id="rank-select"
        value={rankList.find(({ value }) => value === rank)}
        isOptionEqualToValue={(option, value) => option?.value === value?.value}
        options={rankList}
        onChange={handleChangeRank}
        renderInput={(params) => <TextField {...params} label="Select a rank" size="small" />}
      />
    </Box>
  );
}

export default RankMenu;
