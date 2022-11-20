import { useEffect, useState } from 'react';

import { Alert, Box, Unstable_Grid2 as Grid, Typography, Avatar } from '@mui/material';
import { useParams, useSearchParams } from 'react-router-dom';

import Runes from 'components/Runes';
import { useGlobalContext } from 'context/global';
import type { AutoCompleteOption, ChampionData } from 'interfaces';
import { getChampionBuild, validateState } from 'utils/utils';

function ChampionInformationDisplay() {
  const { state } = useGlobalContext();
  const { champion = '' } = useParams();
  const [searchParams] = useSearchParams();
  const [error, setError] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [championInfo, setChampionInfo] = useState<ChampionData | null>(null);
  const [url, setUrl] = useState<string | null>(null);

  useEffect(() => {
    const handleGetChampionInformation = async () => {
      setError('');
      setChampionInfo(null);
      setLoading(true);
      setUrl(null);

      const championValue: { id: number; key: string } = Object.fromEntries(
        [...searchParams.entries()].map(([key, value]) => (key === 'id' ? [key, Number(value)] : [key, value]))
      );

      const selectedChampion: AutoCompleteOption<{ id: number; key: string }> = {
        label: champion,
        value: championValue,
      };

      const newState = { ...state, champion: selectedChampion };

      const stateValidation = validateState(newState);

      if (!stateValidation.isValid) {
        setError(stateValidation.message);
        setLoading(false);

        return;
      }

      console.log(selectedChampion);

      const championInfoResponse = await getChampionBuild(newState);

      console.log(championInfoResponse);

      if (championInfoResponse.completedSuccessfully) {
        const { completedSuccessfully: _, ...restChampionInfo } = championInfoResponse;
        setChampionInfo(restChampionInfo);
        setUrl(`../champions${championInfoResponse?.localImage}`);
      } else {
        setError(championInfoResponse.message);
      }

      setLoading(false);
    };

    handleGetChampionInformation();
  }, [state, champion, searchParams]);

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
                    <Avatar
                      src={url || ''}
                      alt={state.champion?.label}
                      imgProps={{ onError: () => setUrl(championInfo?.url || '') }}
                    />
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
