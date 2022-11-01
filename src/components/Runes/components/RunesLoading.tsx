import { Divider, Paper, Unstable_Grid2 as Grid } from '@mui/material';

import { createArrayFromLength } from 'utils/utils';

import RuneLoading from './RuneLoading';

function RunesLoading() {
  return (
    <Paper elevation={3} sx={{ padding: '10px', maxHeight: '425px', maxWidth: '475px' }}>
      <Grid container>
        <Grid container sm={6}>
          <Grid container sm={12} justifyContent="center">
            {createArrayFromLength(3).map(RuneLoading)}
          </Grid>
          <Grid sm={12}>
            <Divider />
          </Grid>
          <Grid container sm={12}>
            {createArrayFromLength(3).map(RuneLoading)}
          </Grid>
          <Grid container sm={12}>
            {createArrayFromLength(3).map(RuneLoading)}
          </Grid>
          <Grid container sm={12}>
            {createArrayFromLength(3).map(RuneLoading)}
          </Grid>
        </Grid>
        <Grid container sm={6}>
          <Grid container>
            <Grid container sm={12} justifyContent="center">
              {createArrayFromLength(3).map(RuneLoading)}
            </Grid>
            <Grid container sm={12}>
              {createArrayFromLength(3).map(RuneLoading)}
            </Grid>
            <Grid container sm={12}>
              {createArrayFromLength(3).map(RuneLoading)}
            </Grid>
          </Grid>
          <Grid sm={12}>
            <Divider />
          </Grid>
          <Grid container>
            <Grid container sm={12} justifyContent="center">
              {createArrayFromLength(3).map(RuneLoading)}
            </Grid>
            <Grid container sm={12}>
              {createArrayFromLength(3).map(RuneLoading)}
            </Grid>
            <Grid container sm={12}>
              {createArrayFromLength(3).map(RuneLoading)}
            </Grid>
          </Grid>
        </Grid>
      </Grid>
    </Paper>
  );
}

export default RunesLoading;
