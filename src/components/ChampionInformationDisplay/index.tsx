import { Alert, Box, Unstable_Grid2 as Grid, Typography } from '@mui/material';

import Runes from 'components/Runes';
import type { ChampionData } from 'interfaces';

interface ChampionInformationDisplayProps {
  championInfo: ChampionData | null;
  error: string;
  loading: boolean;
}

function ChampionInformationDisplay({ championInfo, error, loading }: ChampionInformationDisplayProps) {
  return (
    <Grid container xs={12}>
      {error ? (
        <Grid xs={12}>
          <Alert color="error">{error}</Alert>
        </Grid>
      ) : (
        <Grid xs={12}>
          <Runes runes={championInfo?.runes} shards={championInfo?.shards} loading={loading || !championInfo} />
        </Grid>
      )}
    </Grid>
  );
}

export default ChampionInformationDisplay;
