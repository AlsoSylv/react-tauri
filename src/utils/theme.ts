import { createTheme } from '@mui/material';

import { performanceValues } from './constants';

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

const colorMap = {
  badPerf: theme.palette.error.main,
  lowPerf: theme.palette.error.light,
  avgPerf: theme.palette.primary.light,
  goodPerf: theme.palette.primary.main,
  bestPerf: theme.palette.warning.light,
  white: theme.palette.text.primary,
};

const colorByPercentage = (percentage: string): string => {
  try {
    const extractedNumber = Number(percentage?.split('%')?.[0] || 0);

    if (extractedNumber < performanceValues.bad) {
      return colorMap.badPerf;
    }

    if (extractedNumber < performanceValues.low) {
      return colorMap.lowPerf;
    }

    if (extractedNumber < performanceValues.avg) {
      return colorMap.avgPerf;
    }

    if (extractedNumber < performanceValues.good) {
      return colorMap.goodPerf;
    }

    return colorMap.bestPerf;
  } catch (error) {
    console.warn('Caught an error while extracting percentage, returning default. ', error);

    return colorMap.white;
  }
};

export { theme, colorByPercentage };
