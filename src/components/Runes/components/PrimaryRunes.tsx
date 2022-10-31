import { Divider, Unstable_Grid2 as Grid } from '@mui/material';

import { PrimaryRunes as IPrimaryRunes } from 'interfaces';
import { runeMap } from 'utils/utils';

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
        {slotOne.map(runeMap)}
      </Grid>
      <Divider />
      <Grid container sm={12}>
        {slotTwo.map(runeMap)}
      </Grid>
      <Grid container sm={12}>
        {slotThree.map(runeMap)}
      </Grid>
      <Grid container sm={12}>
        {slotFour.map(runeMap)}
      </Grid>
    </Grid>
  );
}

export default PrimaryRunes;
