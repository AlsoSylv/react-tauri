import { createTheme, CssBaseline, ThemeProvider } from '@mui/material';

import Container from 'components/Container';
import { GlobalStateProvider } from 'context/global';
import MainPage from 'pages/MainPage';

import './style.css';

const theme = createTheme({
  palette: {
    mode: 'dark',
  },
  typography: {
    fontFamily: [
      'Poppins',
      '-apple-system',
      'BlinkMacSystemFont',
      '"Segoe UI"',
      'Roboto',
      '"Helvetica Neue"',
      'Arial',
      'sans-serif',
      '"Apple Color Emoji"',
      '"Segoe UI Emoji"',
      '"Segoe UI Symbol"',
    ].join(','),
  },
});

function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline enableColorScheme />
      <Container>
        <GlobalStateProvider>
          <MainPage />
        </GlobalStateProvider>
      </Container>
    </ThemeProvider>
  );
}

export default App;
