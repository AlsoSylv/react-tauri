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
    <Grid container xs={6}>
      <Grid container xs={12} justifyContent="center">
        {slotOne.map(Rune)}
      </Grid>
      <Grid xs={12} sx={{ display: 'flex', flexDirection: 'column', justifyContent: 'center' }}>
        <Divider />
      </Grid>
      <Grid container xs={12}>
        {slotTwo.map(Rune)}
      </Grid>
      <Grid container xs={12}>
        {slotThree.map(Rune)}
      </Grid>
      <Grid container xs={12}>
        {slotFour.map(Rune)}
      </Grid>
    </Grid>
  );
}

export default PrimaryRunes;
