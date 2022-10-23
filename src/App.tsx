import { Container, CssBaseline } from '@mui/material';

import { GlobalStateProvider } from 'context/global';
import MainPage from 'pages/MainPage';

function App() {
  return (
    <>
      <CssBaseline enableColorScheme />
      <Container sx={{ height: '100%' }}>
        <GlobalStateProvider>
          <MainPage />
        </GlobalStateProvider>
      </Container>
    </>
  );
}

export default App;
