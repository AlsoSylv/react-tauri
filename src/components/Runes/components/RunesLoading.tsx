import { Divider, Unstable_Grid2 as Grid } from '@mui/material';

import { createArrayFromLength } from 'utils/utils';

import RuneLoading from './RuneLoading';

function RunesLoading() {
  return (
    <Grid container sx={{ flexDirection: 'row', height: '100%' }}>
      <Grid container xs={6}>
        <Grid container xs={12}>
          {createArrayFromLength(3).map(RuneLoading)}
        </Grid>
        <Grid xs={12} sx={{ display: 'flex', flexDirection: 'column', justifyContent: 'center' }}>
          <Divider />
        </Grid>
        <Grid container xs={12}>
          {createArrayFromLength(3).map(RuneLoading)}
        </Grid>
        <Grid container xs={12}>
          {createArrayFromLength(3).map(RuneLoading)}
        </Grid>
        <Grid container xs={12}>
          {createArrayFromLength(3).map(RuneLoading)}
        </Grid>
      </Grid>
      <Grid container xs={6}>
        <Grid container xs={12}>
          <Grid container xs={12}>
            {createArrayFromLength(3).map(RuneLoading)}
          </Grid>
          <Grid container xs={12}>
            {createArrayFromLength(3).map(RuneLoading)}
          </Grid>
          <Grid container xs={12}>
            {createArrayFromLength(3).map(RuneLoading)}
          </Grid>
        </Grid>
        <Grid xs={12} sx={{ display: 'flex', flexDirection: 'column', justifyContent: 'center' }}>
          <Divider />
        </Grid>
        <Grid container xs={12}>
          <Grid container xs={12}>
            {createArrayFromLength(3).map(RuneLoading)}
          </Grid>
          <Grid container xs={12}>
            {createArrayFromLength(3).map(RuneLoading)}
          </Grid>
          <Grid container xs={12}>
            {createArrayFromLength(3).map(RuneLoading)}
          </Grid>
        </Grid>
      </Grid>
    </Grid>
  );
}

export default RunesLoading;
