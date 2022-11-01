import { Unstable_Grid2 as Grid } from '@mui/material';

import { Shards as IShards } from 'interfaces';

import Rune from './Rune';

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
        {rowOne.map(Rune)}
      </Grid>
      <Grid container sm={12}>
        {rowTwo.map(Rune)}
      </Grid>
      <Grid container sm={12}>
        {rowThree.map(Rune)}
      </Grid>
    </Grid>
  );
}

export default Shards;
