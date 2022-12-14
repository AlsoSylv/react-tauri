/* eslint-disable no-console */
import { DDRAGON_URL, DDRAGON_VERSION } from './constants.js';
import { delayedPromise, saveImage } from './utils.js';

const getChampionImages = async (champions = []) => {
  console.time('Get Champion Images');
  const championsData = champions.map(async (championName) => {
    try {
      const championResponse = await fetch(`${DDRAGON_URL}/cdn/${DDRAGON_VERSION}/data/en_US/champion/${championName}.json`);
      const {
        data: {
          [championName]: {
            image: { full: image },
            spells = [],
            passive: {
              image: { full: passiveImage },
            },
          },
        },
      } = await championResponse.json();

      const championFolder = `champions/${championName}`;

      const spellImages = spells.flatMap(({ image: { full }, id }) =>
        delayedPromise(async () => saveImage(`${DDRAGON_VERSION}/img/spell/${full}`, championFolder, `${id}.png`))
      );

      await Promise.all(
        [
          delayedPromise(async () => saveImage(`${DDRAGON_VERSION}/img/champion/${image}`, championFolder, image)),
          delayedPromise(async () => saveImage(`${DDRAGON_VERSION}/img/passive/${passiveImage}`, championFolder, passiveImage)),
        ].concat(spellImages)
      );

      return Promise.resolve();
    } catch (error) {
      console.log('on: ', championName, error);
      return Promise.reject(error);
    }
  });

  await Promise.all(championsData);
  console.timeEnd('Get Champion Images');
};

export default getChampionImages;
