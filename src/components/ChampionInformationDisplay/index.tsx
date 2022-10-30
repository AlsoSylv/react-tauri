import { useState } from 'react';

import { Alert, Box, Button, Unstable_Grid2 as Grid, Typography } from '@mui/material';

import Runes from 'components/Runes';
import { useGlobalContext } from 'context/global';
import { ChampionInfo } from 'interfaces/ChampionInfo';
import { getChampionInfo, validateState } from 'utils/utils';

function ChampionInformationDisplay() {
  const { state } = useGlobalContext();
  const [error, setError] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [championInfo, setChampionInfo] = useState<ChampionInfo | null>(null);

  const handleGetChampionInformation = async () => {
    setError('');
    setChampionInfo(null);
    setLoading(true);

    const stateValidation = validateState(state);

    if (!stateValidation.isValid) {
      setError(stateValidation.message);
      setLoading(false);

      return;
    }

    const championInfoResponse = await getChampionInfo(state);

    if (championInfoResponse.completedSuccessfully) {
      const { completedSuccessfully: _, ...restChampionInfo } = championInfoResponse;
      setChampionInfo(restChampionInfo);
    } else {
      setError(championInfoResponse.message);
    }

    setLoading(false);
  };

  return (
    <Box id="get-runes">
      <Grid container spacing={2}>
        <Grid xs={3}>
          <Button onClick={handleGetChampionInformation} disabled={loading} variant="contained">
            Find runes
          </Button>
        </Grid>
        <Grid xs={4} sx={{ display: 'flex' }}>
          <Typography variant="body1" alignSelf="center">
            Champion Win Rate: {championInfo?.winRate}
          </Typography>
        </Grid>
        <Grid xs={12}>
          <Grid container spacing={2}>
            {error ? (
              <Grid xs={12}>
                <Alert color="error">{error}</Alert>
              </Grid>
            ) : (
              championInfo && <Runes runes={championInfo.runes} shards={championInfo.shards} />
            )}
          </Grid>
        </Grid>
      </Grid>
    </Box>
  );
}

export default ChampionInformationDisplay;
