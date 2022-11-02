import { useEffect, useState } from 'react';

import { Alert, Box, Unstable_Grid2 as Grid, Typography, Avatar } from '@mui/material';
import { useParams } from 'react-router-dom';

import Runes from 'components/Runes';
import { useGlobalContext } from 'context/global';
import { CompleteChampionInfo } from 'interfaces/ChampionInfo';
import { getChampionInfo, validateState } from 'utils/utils';

function ChampionInformationDisplay() {
  const { state } = useGlobalContext();
  const { champion = '' } = useParams();
  const [error, setError] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [championInfo, setChampionInfo] = useState<CompleteChampionInfo | null>(null);

  useEffect(() => {
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

      const championInfoResponse = await getChampionInfo({ ...state, champion });

      if (championInfoResponse.completedSuccessfully) {
        const { completedSuccessfully: _, ...restChampionInfo } = championInfoResponse;
        setChampionInfo(restChampionInfo);
      } else {
        setError(championInfoResponse.message);
      }

      setLoading(false);
    };

    handleGetChampionInformation();
  }, [state]);

  return (
    <Box id="get-runes">
      <Grid container spacing={2}>
        <Grid xs>
          <Grid container spacing={2}>
            {error ? (
              <Grid xs={12}>
                <Alert color="error">{error}</Alert>
              </Grid>
            ) : (
              <>
                <Grid container sx={{ display: 'flex' }}>
                  <Grid xs={1}>
                    <Avatar src={championInfo?.url} alt={state.champion} />
                  </Grid>
                  <Grid xs={3}>
                    <Typography variant="body1" alignSelf="center">
                      Champion Win Rate: {championInfo?.winRate}
                    </Typography>
                  </Grid>
                  <Grid xs={3}>
                    <Typography variant="body1" alignSelf="center">
                      Champion Pick Rate: {championInfo?.pickRate}
                    </Typography>
                  </Grid>
                  <Grid xs={3}>
                    <Typography variant="body1" alignSelf="center">
                      Champion Ban Rate: {championInfo?.banRate}
                    </Typography>
                  </Grid>
                </Grid>
                <Grid xs={12}>
                  <Runes runes={championInfo?.runes} shards={championInfo?.shards} loading={loading || !championInfo} />
                </Grid>
              </>
            )}
          </Grid>
        </Grid>
      </Grid>
    </Box>
  );
}

export default ChampionInformationDisplay;
