import { useEffect, useState } from 'react';

import { Paper, Unstable_Grid2 as Grid, Button, Typography, Skeleton, Box } from '@mui/material';
import { useNavigate, useParams, useSearchParams } from 'react-router-dom';

import ChampionInformationDisplay from 'components/ChampionInformationDisplay';
import ChampionOptions from 'components/ChampionSelection';
import RankMenu from 'components/RankMenu';
import RegionMenu from 'components/RegionMenu';
import RoleMenu from 'components/RoleMenu';
import { useGlobalContext } from 'context/global';
import type { AutoCompleteOption, ChampionData } from 'interfaces';
import { getChampionBuild, validateState } from 'utils/utils';

function Champion() {
  const navigate = useNavigate();
  const { state } = useGlobalContext();
  const { champion = '' } = useParams();
  const [searchParams] = useSearchParams();
  const [error, setError] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(true);
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

      console.log('selectedChampion', selectedChampion);

      const championInfoResponse = await getChampionBuild(newState);

      console.log('championInfoResponse', championInfoResponse);

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

  const goToMainPage = () => navigate('/');

  return (
    <Paper elevation={3} sx={{ height: '100%' }}>
      <Grid container sx={{ height: '100%' }} rowGap={2} component={Paper} elevation={0} direction="column">
        <Grid container xs={12} sx={(theme) => ({ padding: '20px 40px', backgroundColor: theme.palette.background.paper })}>
          <Grid xs={3}>
            <ChampionOptions />
          </Grid>
          <Grid xs={4}>
            <Button onClick={() => setLoading((prev) => !prev)}>Main Page</Button>
          </Grid>
        </Grid>
        <Grid
          container
          xs={12}
          gap={2}
          sx={(theme) => ({ padding: '0 40px', backgroundColor: theme.palette.background.paper })}
        >
          <Grid xs="auto">
            <Box sx={{ maxHeight: 120 }}>
              {loading ? (
                <Skeleton variant="rectangular" width="120px" height="120px" sx={{ borderRadius: '.4rem' }} animation="wave" />
              ) : (
                <img
                  src={url || ''}
                  alt={state.champion?.label}
                  onError={() => setUrl(championInfo?.url || '')}
                  style={{ maxHeight: '120px', borderRadius: '.4rem' }}
                />
              )}
            </Box>
          </Grid>
          <Grid xs container>
            <Grid container xs={12} gap={2}>
              <Grid xs="auto">
                <Box sx={{ display: 'flex', flexDirection: 'row', gap: '20px' }}>
                  <Typography variant="h3" sx={{ fontWeight: 500 }}>
                    {champion}
                  </Typography>
                  <Typography variant="h4" alignSelf="center">
                    Build for {}
                  </Typography>
                </Box>
              </Grid>
              <Grid xs={12} container columnGap={2} justifyContent="flex-end" alignItems="flex-end">
                <Grid xs={3}>
                  <RoleMenu loading={loading} />
                </Grid>
                <Grid xs={3}>
                  <RankMenu />
                </Grid>
                <Grid xs={3}>
                  <RegionMenu />
                </Grid>
              </Grid>
            </Grid>
          </Grid>
        </Grid>
        <Grid xs={12} container gap={2} sx={{ padding: '25px', flexGrow: 1, maxHeight: '100%' }}>
          <ChampionInformationDisplay championInfo={championInfo} error={error} loading={loading} />
        </Grid>
      </Grid>
    </Paper>
  );
}

export default Champion;