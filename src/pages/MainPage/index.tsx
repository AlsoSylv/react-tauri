import { Box, Unstable_Grid2 as Grid } from '@mui/material';

import ChampionOptions from 'components/ChampionSelection';

function MainPage() {
  return (
    <Box>
      <Grid container spacing={2} sx={{ minWidth: '600px' }}>
        <Grid xs={12}>
          <ChampionOptions />
        </Grid>
      </Grid>
    </Box>
  );
}

export default MainPage;
