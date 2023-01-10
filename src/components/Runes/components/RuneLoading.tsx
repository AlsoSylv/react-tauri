import { Avatar, Skeleton, Unstable_Grid2 as Grid } from '@mui/material';

function RuneLoading(tempId: string) {
  return (
    <Grid key={`rune-loading-${tempId}`} xs sx={{ display: 'flex', alignSelf: 'center', justifyContent: 'center' }}>
      <Skeleton variant="circular" width={40} height={40} animation="wave" />
    </Grid>
  );
}

export default RuneLoading;
