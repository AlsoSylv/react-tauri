import { Unstable_Grid2 as Grid } from '@mui/material';

import { SecondaryRunes as ISecondaryRunes } from 'interfaces';

import Rune from './Rune';

interface SecondaryRunesProps {
  secondaryRunes: ISecondaryRunes;
}

function SecondaryRunes(props: SecondaryRunesProps) {
  const {
    secondaryRunes: { slotOne, slotTwo, slotThree },
  } = props;

  return (
    <Grid container>
      <Grid container sm={12}>
        {slotOne.map(Rune)}
      </Grid>
      <Grid container sm={12}>
        {slotTwo.map(Rune)}
      </Grid>
      <Grid container sm={12}>
        {slotThree.map(Rune)}
      </Grid>
    </Grid>
  );
}

export default SecondaryRunes;
