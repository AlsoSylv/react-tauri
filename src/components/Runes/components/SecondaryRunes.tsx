import { Unstable_Grid2 as Grid } from '@mui/material';

import { SecondaryRunes as ISecondaryRunes } from 'interfaces';
import { runeMap } from 'utils/utils';

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
        {slotOne.map(runeMap)}
      </Grid>
      <Grid container sm={12}>
        {slotTwo.map(runeMap)}
      </Grid>
      <Grid container sm={12}>
        {slotThree.map(runeMap)}
      </Grid>
    </Grid>
  );
}

export default SecondaryRunes;
