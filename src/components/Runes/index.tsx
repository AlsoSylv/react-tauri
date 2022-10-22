import { useState } from 'react';

import { Alert, Box, Button, Unstable_Grid2 as Grid, Chip } from '@mui/material';

import { useGlobalContext } from 'context/global';
import { getRunes, validateState } from 'utils/';

function Runes() {
  const { state } = useGlobalContext();
  const [error, setError] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [runes, setRunes] = useState<Array<Array<string>> | null>(null);

  const handleGetRunes = async () => {
    setError('');
    setRunes(null);
    setLoading(true);

    const stateValidation = validateState(state);

    if (!stateValidation.isValid) {
      setError(stateValidation.message);
      setLoading(false);

      return;
    }

    const matchingRunes = await getRunes(state);

    if (matchingRunes.completedSuccessfully) {
      setRunes(matchingRunes.runes);
    } else {
      setError(matchingRunes.message);
    }

    setLoading(false);
  };

  return (
    <Box id="get-runes">
      <Grid container spacing={2}>
        <Grid xs={3}>
          <Button onClick={handleGetRunes} disabled={loading} variant="contained">
            Find runes
          </Button>
        </Grid>
        {error && (
          <Grid xs={12}>
            <Alert color="error">{error}</Alert>
          </Grid>
        )}
        <Grid xs={12}>
          <Grid container>
            {runes?.map((runeRow) =>
              runeRow.map((rune) => (
                <Grid key={rune}>
                  <Chip label={rune} />
                </Grid>
              ))
            )}
          </Grid>
        </Grid>
      </Grid>
    </Box>
  );
}

export default Runes;
