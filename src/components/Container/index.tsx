import type { PropsWithChildren } from 'react';

import { Box } from '@mui/material';

function Container({ children }: PropsWithChildren) {
  return (
    <Box
      sx={{
        height: '100%',
        width: '100%',
        display: 'flex',
        alignContent: 'center',
        justifyContent: 'center',
        flexWrap: 'wrap',
        padding: '20px',
      }}
    >
      {children}
    </Box>
  );
}

export default Container;
