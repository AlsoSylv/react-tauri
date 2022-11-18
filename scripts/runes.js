/* eslint-disable no-console */
import { delayedPromise, saveImage } from './utils.js';

const getRuneImages = async (runePaths) => {
  console.time('Get Runes Images');
  const completeRunes = runePaths.flatMap(({ slots, icon: runePathIcon, key: runePathName }) => {
    const runePath = [
      delayedPromise(async () => saveImage(`img/${runePathIcon}`, `runes/${runePathName}`, `${runePathName}.png`)),
    ];

    const runesImages = slots.flatMap(({ runes }) =>
      runes.map(({ icon, key }) => delayedPromise(async () => saveImage(`img/${icon}`, `runes/${runePathName}`, `${key}.png`)))
    );

    return runePath.concat(runesImages);
  });

  await Promise.allSettled(completeRunes);
  console.timeEnd('Get Runes Images');
};

export default getRuneImages;
