import { Divider, Paper, Unstable_Grid2 as Grid } from '@mui/material';

import type { RuneTrees, Shards as IShards } from 'interfaces';

import { PrimaryRunes, SecondaryRunes, Shards } from './components';
import RunesLoading from './components/RunesLoading';

interface RunesProps {
  runes: RuneTrees | undefined;
  shards: IShards | undefined;
  loading: boolean;
}

function Runes(props: RunesProps) {
  const { runes, shards, loading } = props;

  if (loading) {
    return <RunesLoading />;
  }

  return (
    <Paper elevation={3} sx={{ padding: '10px', maxHeight: '425px', maxWidth: '475px' }}>
      <Grid container>
        <PrimaryRunes primaryRunes={runes!.primaryRunes} />
        <Grid container sm={6}>
          <SecondaryRunes secondaryRunes={runes!.secondaryRunes} />
          <Grid sm={12}>
            <Divider />
          </Grid>
          <Shards shards={shards!} />
        </Grid>
      </Grid>
    </Paper>
  );
}

export default Runes;
