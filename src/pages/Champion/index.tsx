import { Box, Unstable_Grid2 as Grid, Button } from '@mui/material';
import { useNavigate } from 'react-router-dom';

import ChampionInformationDisplay from 'components/ChampionInformationDisplay';
import RankMenu from 'components/RankMenu';
import RegionMenu from 'components/RegionMenu';
import RoleMenu from 'components/RoleMenu';

function Champion() {
  const navigate = useNavigate();

  const goToMainPage = () => navigate('/');

  return (
    <Box>
      <Grid container spacing={2}>
        <Grid xs={4}>
          <Button onClick={goToMainPage}>Main Page</Button>
        </Grid>
      </Grid>
      <Grid container spacing={2}>
        <Grid xs={4}>
          <RoleMenu />
        </Grid>
        <Grid xs={4}>
          <RegionMenu />
        </Grid>
        <Grid xs={4}>
          <RankMenu />
        </Grid>
        <Grid xs={12}>
          <ChampionInformationDisplay />
        </Grid>
      </Grid>
    </Box>
  );
}

export default Champion;
