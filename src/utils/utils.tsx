import { invoke } from '@tauri-apps/api';

import {
  ChampionInfoResponse,
  State,
  ChampionOptions,
  ChampionInfo,
  ChampionBuild,
  ValidatedStateResponse,
  AutoCompleteOption,
} from 'interfaces';

import errors from './errors';

async function getChampionBuild(state: State): Promise<ChampionInfoResponse> {
  const requestArgs = {
    name: state.champion,
    role: state.role,
    rank: state.rank,
    region: state.region,
    lang: state.selectedLanguage,
  };
  try {
    const [championBuild, championInfo] = await Promise.all([
      invoke<ChampionBuild>('runes_and_abilities', requestArgs),
      invoke<ChampionInfo>('champion_info', requestArgs),
    ]);

    return { ...championInfo, ...championBuild, completedSuccessfully: true };
  } catch (exception) {
    const parsedNumber = Number(exception);
    const error = errors[parsedNumber];
    const errorMessage = error?.message || 'No Data Exists!';

    console.error('Got an error while trying to fetch the champion build for state %o', requestArgs);
    console.error(error || exception);

    return { message: errorMessage, completedSuccessfully: false };
  }
}

function getChampionNames(lang: string) {
  return invoke<ChampionOptions[]>('all_champion_names', { lang });
}

const getLanguageName = (baseLanguage: string, languageCode: string): string =>
  new Intl.DisplayNames([baseLanguage], { type: 'language', languageDisplay: 'standard' }).of(languageCode.replace('_', '-')) ||
  '';

const fixLanguageCode = (language: string): string => language.replace('_', '-');

const unfixLanguageCode = (language: string): string => language.replace('-', '_');

const capitalize = (string: string): string =>
  string?.length > 1 ? string.charAt(0).toUpperCase() + string.slice(1) : string.charAt(0).toUpperCase();

const getLanguageList = (currentLanguage: string, languages: string[]): AutoCompleteOption<string>[] => {
  const fixedLanguage = currentLanguage.replace('_', '-');

  return languages.map((language) => ({
    label: capitalize(getLanguageName(fixedLanguage, fixLanguageCode(language))),
    value: language,
  }));
};

const getSystemLanguage = () => unfixLanguageCode(Intl.DateTimeFormat().resolvedOptions().locale);

const validateState = (state: State): ValidatedStateResponse => {
  const { champion, role } = state;

  if (!champion) {
    return { isValid: false, message: 'Please Enter A Champion Name' };
  }

  if (role !== 'default' && role === '') {
    return { isValid: false, message: 'Please Select a Role' };
  }

  return { isValid: true, message: '' };
};

const createArrayFromLength = (length: number) => [...Array(length)].map((_) => Math.ceil(Math.random() * 40 * length));

export {
  getChampionBuild,
  getChampionNames,
  validateState,
  createArrayFromLength,
  getLanguageList,
  fixLanguageCode,
  unfixLanguageCode,
  getSystemLanguage,
};
