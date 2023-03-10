/* eslint-disable no-console */
import { invoke } from '@tauri-apps/api';

import {
  ChampionInfoResponse,
  State,
  ChampionOptions,
  ChampionBuild,
  ValidatedStateResponse,
  AutoCompleteOption,
  InitialData,
  CachedData,
  InvokeAndCacheProps,
  Response,
} from 'interfaces';
import Role from 'interfaces/Role';

import { errors, DEFAULT_CACHE_DURATION } from './constants';
import InvalidRequestState from './errors';

async function getChampionBuild(state: State): Promise<ChampionInfoResponse> {
  const requestArgs = {
    name: state.champion,
    rank: state.rank,
    region: state.region,
    lang: state.selectedLanguage,
    ...(state.role ? { role: state.role } : {}),
  };
  try {
    const championBuild = await invoke<Response<ChampionBuild>>('champion_info', requestArgs);

    const { data, invalidData } = extractDataFromOption(championBuild);

    if (invalidData.length > 0) {
      throw new InvalidRequestState('The request failed due to the following fields failed by', invalidData);
    }

    console.log('extractedData', data);

    return { ...data, completedSuccessfully: true };
  } catch (exception) {
    console.log(exception);
    if (exception instanceof InvalidRequestState) {
      const fieldsString = exception.invalidFields
        .map(({ errorCode, field }) => `${field}: ${getErrorMessage(errorCode)}`)
        .join(',\n');

      console.error('Got an error while trying to fetch the champion build for state %o', requestArgs);
      console.error(fieldsString);
    } else {
      console.error('Got an error while trying to fetch the champion build for state %o', requestArgs);
      console.error(exception);
    }

    return { message: 'The request failed.', completedSuccessfully: false };
  }
}

function extractDataFromOption<T>(data: Response<T>) {
  return Object.entries(data).reduce(
    (acc, [key, value]) => {
      if (typeof value === 'object') {
        if (Object.hasOwn(value, 'Ok')) {
          acc.data[key as keyof T] = value.Ok;
        } else {
          acc.invalidData.push({ field: key, errorCode: value.Err });
        }
      }

      return acc;
    },
    { data: {} as T, invalidData: [] as { field: string; errorCode: number }[] }
  );
}

function getErrorMessage(number: number) {
  const error = errors[number];
  return error?.message || 'No Data Exists!';
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
  const { champion } = state;

  if (!champion) {
    return { isValid: false, message: 'Please Enter A Champion Name' };
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
    const languages = await invokeAndCache<string[]>({ method: 'get_languages' });

    const systemLanguage = getCurrentLanguage();
    const selectedLanguage = languages.find((language) => fixLanguageCode(systemLanguage) === language) || languages[0];

    const languageList = getLanguageList(selectedLanguage, languages);

    const [roleList, tiers, regions] = await Promise.all([
      invokeAndCache<Role[]>({ method: 'roles', args: { lang: selectedLanguage } }),
      invokeAndCache<Record<string, string>>({ method: 'tiers', args: { lang: selectedLanguage } }),
      invokeAndCache<Record<string, string>>({ method: 'regions', args: { lang: selectedLanguage } }),
    ]);

    console.log(tiers);
    console.log(regions);

    const rankList: AutoCompleteOption<string>[] = Object.values(tiers).map((tier) => ({ label: tier, value: tier }));
    const regionList: AutoCompleteOption<string>[] = Object.values(regions).map((region) => ({ label: region, value: region }));

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
