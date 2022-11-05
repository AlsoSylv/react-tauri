/* eslint-disable no-console */

import getChampionImages from './champions.js';
import { DDRAGON_URL, DDRAGON_VERSION } from './constants.js';
import getItemImages from './items.js';
import getRuneImages from './runes.js';

try {
  const runesResponse = await fetch(`${DDRAGON_URL}/cdn/${DDRAGON_VERSION}/data/en_US/runesReforged.json`);
  const runePaths = await runesResponse.json();

  const itemsResponse = await fetch(`${DDRAGON_URL}/cdn/${DDRAGON_VERSION}/data/en_US/item.json`);
  const { data: items } = await itemsResponse.json();

  const championsResponse = await fetch(`${DDRAGON_URL}/cdn/${DDRAGON_VERSION}/data/en_US/champion.json`);
  const { data: champions } = await championsResponse.json();

  const championsList = Object.keys(champions);

  await getRuneImages(runePaths);
  await getItemImages(items);
  await getChampionImages(championsList);

  console.log('Completed data fetching successfully.');
} catch (err) {
  console.error('Failed to fetch all images due to: ', err);
}
