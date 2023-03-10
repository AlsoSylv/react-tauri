import { Unstable_Grid2 as Grid } from '@mui/material';

import ChampionOptions from 'components/ChampionSelection';

function MainPage() {
  return (
    <Grid container sx={{ minWidth: '600px' }}>
      <Grid xs={12}>
        <ChampionOptions />
      </Grid>
    </Grid>
  );
}

export default MainPage;
