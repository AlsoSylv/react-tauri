import { useEffect, useState } from 'react';

import { Box, Unstable_Grid2 as Grid, CircularProgress } from '@mui/material';
import { invoke } from '@tauri-apps/api';

import ChampionOptions from 'components/ChampionSelection';
import { useGlobalContext } from 'context/global';
import { Actions } from 'context/global/actions';
import type { AutoCompleteOption } from 'interfaces';
import { fixLanguageCode, getLanguageList, getSystemLanguage } from 'utils/utils';

function MainPage() {
  const [isLoading, setIsLoading] = useState(true);
  const { dispatch } = useGlobalContext();

  useEffect(() => {
    const getInitialData = async () => {
      setIsLoading(true);
      const [roles, tiers, regions, languages] = await Promise.all([
        invoke<string[]>('roles'),
        invoke<string[]>('tiers'),
        invoke<string[]>('regions'),
        invoke<string[]>('get_languages'),
      ]);

      const systemLanguage = getSystemLanguage();
      const selectedLanguage = languages.find((language) => fixLanguageCode(systemLanguage) === language) || languages[0];
      const languageList = getLanguageList(selectedLanguage, languages);

      const roleList: AutoCompleteOption<string>[] = roles.map((role) => ({ label: role, value: role }));
      const rankList: AutoCompleteOption<string>[] = tiers.map((tier) => ({ label: tier, value: tier }));
      const regionList: AutoCompleteOption<string>[] = regions.map((region) => ({ label: region, value: region }));

      const payload = { roleList, rankList, regionList, languageList };

      dispatch({ type: Actions.SET_INITIAL_DATA, payload });
      dispatch({ type: Actions.SET_SELECTED_LANGUAGE, payload: selectedLanguage });

      setIsLoading(false);
    };

    getInitialData();
  }, [dispatch]);

  return (
    <Box>
      {isLoading ? (
        <CircularProgress />
      ) : (
        <Grid container spacing={2} sx={{ minWidth: '600px' }}>
          <Grid xs={12}>
            <ChampionOptions />
          </Grid>
        </Grid>
      )}
    </Box>
  );
}

export default MainPage;
