import { Box, Unstable_Grid2 as Grid } from '@mui/material';

import ChampionOptions from 'components/ChampionSelection';
import RankMenu from 'components/RankMenu';
import RegionMenu from 'components/RegionMenu';
import RoleMenu from 'components/RoleMenu';
import Runes from 'components/Runes';

function MainPage() {
  return (
    <Box sx={{ marginTop: '20px' }}>
      <Grid container spacing={2}>
        <Grid xs={3}>
          <ChampionOptions />
        </Grid>
        <Grid xs={3}>
          <RoleMenu />
        </Grid>
        <Grid xs={3}>
          <RegionMenu />
        </Grid>
        <Grid xs={3}>
          <RankMenu />
        </Grid>
        <Grid xs={12}>
          <Runes />
        </Grid>
      </Grid>
    </Box>
  );
}

export default MainPage;
