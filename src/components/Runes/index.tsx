import { Divider, Paper, Unstable_Grid2 as Grid } from '@mui/material';

import type { Runes as IRunes, Shards as IShards } from 'interfaces';

import { PrimaryRunes, SecondaryRunes, Shards } from './components';
import RunesLoading from './components/RunesLoading';

type RunesProps =
  | {
      runes: IRunes;
      shards: IShards;
      loading: false;
    }
  | {
      runes: undefined;
      shards: undefined;
      loading: true;
    };

function Runes(props: RunesProps) {
  const { runes, shards, loading } = props;

  if (loading) {
    return <RunesLoading />;
  }

  return (
    <Paper elevation={3} sx={{ padding: '10px', aspectRatio: '16 / 9', height: '375px' }}>
      <Grid container xs={12} sx={{ flexDirection: 'row', height: '100%' }}>
        <PrimaryRunes primaryRunes={runes.primaryRunes} />
        <Grid container xs={6}>
          <SecondaryRunes secondaryRunes={runes.secondaryRunes} />
          <Grid xs={12} sx={{ display: 'flex', flexDirection: 'column', justifyContent: 'center' }}>
            <Divider />
          </Grid>
          <Shards shards={shards} />
        </Grid>
      </Grid>
    </Paper>
  );
}

export default Runes;
