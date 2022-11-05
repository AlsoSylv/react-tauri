/* eslint-disable no-console */
import { DDRAGON_VERSION } from './constants.js';
import { delayedPromise, saveImage } from './utils.js';

const getItemImages = async (items = []) => {
  console.time('Get Item Images');
  const completeItems = Object.keys(items).map((key) =>
    delayedPromise(async () => saveImage(`${DDRAGON_VERSION}/img/item/${items[key].image.full}`, `items`, `${key}.png`))
  );

  await Promise.allSettled(completeItems);
  console.timeEnd('Get Item Images');
};

export default getItemImages;
