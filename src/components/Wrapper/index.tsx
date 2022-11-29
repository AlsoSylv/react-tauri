import { CssBaseline, ThemeProvider } from '@mui/material';
import { Outlet } from 'react-router-dom';

import Container from 'components/Container';
import Layout from 'components/Layout';
import { GlobalStateProvider } from 'context/global';
import theme from 'utils/theme';

function Wrapper() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline enableColorScheme />
      <GlobalStateProvider>
        <Container>
          <Layout>
            <Outlet />
          </Layout>
        </Container>
      </GlobalStateProvider>
    </ThemeProvider>
  );
}

export default Wrapper;
