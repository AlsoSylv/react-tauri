import { Divider, Unstable_Grid2 as Grid } from '@mui/material';

import { PrimaryRunes as IPrimaryRunes } from 'interfaces';

import Rune from './Rune';

interface PrimaryRunesProps {
  primaryRunes: IPrimaryRunes;
}

function PrimaryRunes(props: PrimaryRunesProps) {
  const {
    primaryRunes: { slotOne, slotTwo, slotThree, slotFour },
  } = props;

  return (
    <Grid container sm={6}>
      <Grid container sm={12} justifyContent="center">
        {slotOne.map(Rune)}
      </Grid>
      <Grid sm={12} sx={{ display: 'flex', flexDirection: 'column', justifyContent: 'center' }}>
        <Divider />
      </Grid>
      <Grid container sm={12}>
        {slotTwo.map(Rune)}
      </Grid>
      <Grid container sm={12}>
        {slotThree.map(Rune)}
      </Grid>
      <Grid container sm={12}>
        {slotFour.map(Rune)}
      </Grid>
    </Grid>
  );
}

export default PrimaryRunes;
