import { SyntheticEvent } from 'react';

import { Autocomplete, Box, TextField } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import { AutoCompleteOption } from 'interfaces';

const ranks: AutoCompleteOption[] = [
  { label: 'Challenger', value: 'challenger' },
  { label: 'Grandmaster', value: 'grandmaster' },
  { label: 'Master', value: 'master' },
  { label: 'Diamond', value: 'diamond' },
  { label: 'Platinum', value: 'platinum' },
  { label: 'Gold', value: 'gold' },
  { label: 'Silver', value: 'silver' },
  { label: 'Bronze', value: 'bronze' },
  { label: 'Iron', value: 'iron' },
  { label: 'All Ranks', value: 'overall' },
  { label: 'Master +', value: 'master_plus' },
  { label: 'Diamond +', value: 'diamond_plus' },
  { label: 'Diamond 2 +', value: 'diamond_2_plus0' },
  { label: 'Platinum +', value: 'platinum_plus' },
];

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
