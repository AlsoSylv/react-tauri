import { Alert, Unstable_Grid2 as Grid, Box, Divider, Skeleton } from '@mui/material';
import Paper from '@mui/material/Paper';
import Typography from '@mui/material/Typography';

import Runes from 'components/Runes';
import type { ChampionData } from 'interfaces';

interface ChampionInformationDisplayProps {
  championInfo: ChampionData | null;
  error: string;
  loading: boolean;
}

function BasicInfoData(props: { label: string; value: string; loading: boolean }) {
  const { label, value, loading } = props;

  return (
    <Box sx={{ display: 'flex', flexDirection: 'column' }}>
      <Typography variant="h6" alignSelf="center" sx={{ fontWeight: 'bold' }}>
        {loading ? <Skeleton width="4rem" /> : value}
      </Typography>
      <Typography variant="body2" alignSelf="center" display="flex">
        {label}
      </Typography>
    </Box>
  );
}

function ChampionInformationDisplay({ championInfo, error, loading }: ChampionInformationDisplayProps) {
  return (
    <Grid container xs={12}>
      {error ? (
        <Grid xs={12}>
          <Alert color="error">{error}</Alert>
        </Grid>
      ) : (
        <>
          <Grid xs={8.5} gap={4}>
            <Runes runes={championInfo?.runes} shards={championInfo?.shards} loading={loading || !championInfo} />
          </Grid>
          <Grid xs={3.5}>
            <Paper elevation={3} sx={{ padding: '20px' }}>
              <Typography variant="body1" sx={{ fontWeight: 500, fontSize: '1.2rem' }} alignSelf="center" gutterBottom>
                Stats Summary
              </Typography>
              <Box>
                <Grid container xs={12} justifyContent="space-evenly">
                  <Grid xs="auto">
                    <BasicInfoData value={championInfo?.winRate || ''} label="Win Rate" loading={loading} />
                  </Grid>
                  <Grid xs="auto">
                    <Divider variant="fullWidth" orientation="vertical" />
                  </Grid>
                  <Grid xs="auto">
                    <BasicInfoData value={championInfo?.pickRate || ''} label="Pick Rate" loading={loading} />
                  </Grid>
                  <Grid xs="auto">
                    <Divider variant="fullWidth" orientation="vertical" />
                  </Grid>
                  <Grid xs="auto">
                    <BasicInfoData value={championInfo?.banRate || ''} label="Ban Rate" loading={loading} />
                  </Grid>
                </Grid>
              </Box>
            </Paper>
          </Grid>
        </>
      )}
    </Grid>
  );
}

export default ChampionInformationDisplay;
