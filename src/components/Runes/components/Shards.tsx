import { Unstable_Grid2 as Grid } from '@mui/material';

import { Shards as IShards } from 'interfaces';
import { runeMap } from 'utils/utils';

interface ShardsProps {
  shards: IShards;
}

function Shards(props: ShardsProps) {
  const {
    shards: { rowOne, rowTwo, rowThree },
  } = props;

  return (
    <Grid container>
      <Grid container sm={12}>
        {rowOne.map(runeMap)}
      </Grid>
      <Grid container sm={12}>
        {rowTwo.map(runeMap)}
      </Grid>
      <Grid container sm={12}>
        {rowThree.map(runeMap)}
      </Grid>
    </Grid>
  );
}

export default Shards;
