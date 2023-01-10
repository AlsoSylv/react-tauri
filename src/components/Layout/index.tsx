import { useState, PropsWithChildren } from 'react';

import { Box, Unstable_Grid2 as Grid } from '@mui/material';

function Layout({ children }: PropsWithChildren) {
  const [isCollapsed, setIsCollapsed] = useState(false);

  return (
    <Grid container sx={{ height: '100%' }}>
      <Grid xs={isCollapsed ? 0.5 : 1} sx={{ height: '100%', transition: 'width 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94)' }}>
        <Grid container sx={{ backgroundColor: 'red', height: '100%' }}>
          <Box>
            <button onClick={() => setIsCollapsed(!isCollapsed)}>click me</button>
          </Box>
        </Grid>
      </Grid>
      <Grid
        xs={isCollapsed ? 11.5 : 11}
        sx={{ height: '100%', transition: 'width 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94)' }}
        gap={6}
      >
        <Grid xs={12} sx={{ height: '100%' }}>
          {children}
        </Grid>
      </Grid>
    </Grid>
  );
}

export default Layout;
