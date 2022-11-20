/* eslint-disable no-console */
import { invoke } from '@tauri-apps/api';

import {
  ChampionInfoResponse,
  State,
  ChampionOptions,
  ChampionInfo,
  ChampionBuild,
  ValidatedStateResponse,
  AutoCompleteOption,
  InitialData,
  CachedData,
  InvokeAndCacheProps,
} from 'interfaces';

import { errors, DEFAULT_CACHE_DURATION } from './constants';

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

function getLanguageName(baseLanguage: string, languageCode: string): string {
  return (
    new Intl.DisplayNames([baseLanguage], { type: 'language', languageDisplay: 'standard' }).of(
      languageCode.replace('_', '-')
    ) || ''
  );
}

function fixLanguageCode(language: string): string {
  return language.replace('_', '-');
}

function unfixLanguageCode(language: string): string {
  return language.replace('-', '_');
}

function capitalize(string: string): string {
  return string?.length > 1 ? string.charAt(0).toUpperCase() + string.slice(1) : string.charAt(0).toUpperCase();
}

function getLanguageList(currentLanguage: string, languages: string[]): AutoCompleteOption<string>[] {
  const fixedLanguage = currentLanguage.replace('_', '-');

  return languages.map((language) => ({
    label: capitalize(getLanguageName(fixedLanguage, fixLanguageCode(language))),
    value: language,
  }));
}

function getSystemLanguage() {
  return unfixLanguageCode(Intl.DateTimeFormat().resolvedOptions().locale);
}

function validateState(state: State): ValidatedStateResponse {
  const { champion, role } = state;

  if (!champion) {
    return { isValid: false, message: 'Please Enter A Champion Name' };
  }

  if (role !== 'default' && role === '') {
    return { isValid: false, message: 'Please Select a Role' };
  }

  return { isValid: true, message: '' };
}

function getRandomKey() {
  return crypto.randomUUID();
}

function createArrayFromLength(length: number) {
  return [...Array(length)].map(getRandomKey);
}

function parseJson<T>(json: string): T | null {
  try {
    return JSON.parse(json);
  } catch (_) {
    return null;
  }
}

async function invokeAndCache<T>(props: InvokeAndCacheProps): Promise<T> {
  const { method, args, cacheDuration = DEFAULT_CACHE_DURATION } = props;

  try {
    const localStorageValue = parseJson<CachedData<T>>(localStorage.getItem(method) || '');

    if (localStorageValue && localStorageValue.validUntil > Date.now()) {
      return localStorageValue.data;
    }

    const value = await invoke<T>(method, args);

    const valueToSave: CachedData<T> = {
      validUntil: Date.now() + cacheDuration,
      data: value,
    };

    localStorage.setItem(method, JSON.stringify(valueToSave));

    return value;
  } catch (error) {
    console.error('Failed to fetch and cache data: ', error);
    throw error;
  }
}

function getCurrentLanguage(): string {
  return localStorage.getItem('language') || getSystemLanguage();
}

async function getInitialData(): Promise<InitialData> {
  try {
    const [roles, tiers, regions, languages] = await Promise.all([
      invokeAndCache<string[]>({ method: 'roles' }),
      invokeAndCache<string[]>({ method: 'tiers' }),
      invokeAndCache<string[]>({ method: 'regions' }),
      invokeAndCache<string[]>({ method: 'get_languages' }),
    ]);

    const systemLanguage = getCurrentLanguage();
    const selectedLanguage = languages.find((language) => fixLanguageCode(systemLanguage) === language) || languages[0];

    const languageList = getLanguageList(selectedLanguage, languages);
    const roleList: AutoCompleteOption<string>[] = roles.map((role) => ({ label: role, value: role }));
    const rankList: AutoCompleteOption<string>[] = tiers.map((tier) => ({ label: tier, value: tier }));
    const regionList: AutoCompleteOption<string>[] = regions.map((region) => ({ label: region, value: region }));

    return { languageList, rankList, roleList, regionList, selectedLanguage };
  } catch (error) {
    console.error('Failed to get initial data, returning empty values: ', error);
    return { languageList: [], rankList: [], roleList: [], regionList: [], selectedLanguage: '' };
  }
}

export {
  getInitialData,
  getCurrentLanguage,
  getChampionBuild,
  getChampionNames,
  validateState,
  createArrayFromLength,
  getLanguageList,
  fixLanguageCode,
  unfixLanguageCode,
  getSystemLanguage,
};
