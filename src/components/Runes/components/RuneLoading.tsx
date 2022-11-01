import { Avatar, Skeleton, Unstable_Grid2 as Grid } from '@mui/material';

function RuneLoading(tempId: number) {
  return (
    <Grid key={`rune-loading-${tempId}`} sm sx={{ display: 'flex', alignSelf: 'center', justifyContent: 'center' }}>
      <Skeleton variant="circular" width={40} height={40} animation="wave">
        <Avatar />
      </Skeleton>
    </Grid>
  );
}

export default RuneLoading;
