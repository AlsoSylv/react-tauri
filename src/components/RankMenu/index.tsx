import { SyntheticEvent } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';
import { invoke } from '@tauri-apps/api';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import AutoCompleteOption from 'interfaces/AutoCompleteOption';

const ranks: AutoCompleteOption[] = [];

invoke<string[]>('tiers').then((tier) => {
  tier.map((Tier) => ranks.push({ label: Tier, value: Tier }));
});

function RankMenu() {
  const {
    state: { rank },
    dispatch,
  } = useGlobalContext();

  const handleChangeRank = (_: SyntheticEvent<Element, Event>, value: AutoCompleteOption | null) => {
    const newValue = value?.value || '';

    dispatch({ type: Actions.UPDATE_RANK, payload: newValue });
  };

  return (
    <Box>
      <Autocomplete
        disablePortal
        disableClearable
        id="rank-select"
        value={ranks.find(({ value }) => value === rank)}
        isOptionEqualToValue={(option, value) => option.value === value.value}
        options={ranks}
        onChange={handleChangeRank}
        renderInput={(params) => <TextField {...params} label="Select a rank" />}
      />
    </Box>
  );
}

export default RankMenu;
