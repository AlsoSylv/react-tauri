import { Divider, Unstable_Grid2 as Grid } from '@mui/material';

import type { Trees, Shards as IShards } from 'interfaces';

import { PrimaryRunes, SecondaryRunes, Shards } from './components';

interface RunesProps {
  runes: Trees;
  shards: IShards;
}

function Runes(props: RunesProps) {
  const { runes, shards } = props;

  return (
    <Grid container>
      <PrimaryRunes primaryRunes={runes.primaryRunes} />
      <Grid container sm={6}>
        <SecondaryRunes secondaryRunes={runes.secondaryRunes} />
        <Divider />
        <Shards shards={shards} />
      </Grid>
    </Grid>
  );
}

export default Runes;
