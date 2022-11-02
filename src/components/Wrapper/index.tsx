import { CssBaseline, ThemeProvider } from '@mui/material';
import { Outlet } from 'react-router-dom';

import Container from 'components/Container';
import { GlobalStateProvider } from 'context/global';
import theme from 'utils/theme';

function Wrapper() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline enableColorScheme />
      <Container>
        <GlobalStateProvider>
          <Outlet />
        </GlobalStateProvider>
      </Container>
    </ThemeProvider>
  );
}

export default Wrapper;
