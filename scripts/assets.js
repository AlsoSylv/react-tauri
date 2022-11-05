/* eslint-disable no-console */
import { createWriteStream } from 'fs';
import { access, mkdir } from 'fs/promises';
import { resolve, join } from 'path';

const rootFolder = resolve('./');

const DDRAGON_URL = 'http://ddragon.leagueoflegends.com';

const dDragonVersion = (await (await fetch(`${DDRAGON_URL}/api/versions.json`)).json())[0];
const runePaths = await (await fetch(`${DDRAGON_URL}/cdn/${dDragonVersion}/data/en_US/runesReforged.json`)).json();
const { data: items } = await (await fetch(`${DDRAGON_URL}/cdn/${dDragonVersion}/data/en_US/item.json`)).json();

async function pathExists(filePath) {
  try {
    await access(filePath);
    return true;
  } catch {
    return false;
  }
}

const stream = (downloadStream) =>
  new WritableStream({
    write(chunk) {
      downloadStream.write(chunk);
    },
    abort(reason) {
      console.error(reason);
    },
  });

async function writeFile(folderName, fileName, data) {
  try {
    const folderDir = join(rootFolder, 'src/assets/', folderName);
    const fileExists = await pathExists(folderDir);

    if (!fileExists) {
      await mkdir(folderDir, { recursive: true });
    }

    const downloadWriteStream = createWriteStream(join(folderDir, fileName));

    data.pipeTo(stream(downloadWriteStream));
  } catch (err) {
    console.log(err);
    throw new Error(err);
  }
}

const getAndSaveImage = async (url, folder, name) => {
  const imgUrl = `${DDRAGON_URL}/cdn/${url}`;
  try {
    const { body } = await fetch(imgUrl);

    await writeFile(folder, `${name}.png`, body);
  } catch (error) {
    console.error('error on: ', imgUrl);
    console.error(error);
    throw new Error(error);
  }
};

const getRandomMs = (minMs, maxMs) =>
  (minMs + ((maxMs - minMs + 1) * crypto.getRandomValues(new Uint32Array(1))[0]) / 2 ** 32) | 0;

try {
  console.time('Get Runes Images');
  const completeRunes = runePaths.flatMap(({ slots, icon: runePathIcon, key: runePathName }) => {
    const runePath = [getAndSaveImage(`img/${runePathIcon}`, `runes/${runePathName}`, runePathName)];

    const runesImages = slots.flatMap(({ runes }) =>
      runes.map(({ icon, key }) => getAndSaveImage(`img/${icon}`, `runes/${runePathName}`, key))
    );

    return runePath.concat(runesImages);
  });

  await Promise.all(completeRunes);
  console.timeEnd('Get Runes Images');

  console.time('Get Item Images');
  const completeItems = Object.keys(items).map(
    (key) =>
      new Promise((r) => {
        setTimeout(async () => {
          await getAndSaveImage(`${dDragonVersion}/img/item/${items[key].image.full}`, `items`, key);
          r();
        }, getRandomMs(0, 1000));
      })
  );

  await Promise.all(completeItems);
  console.timeEnd('Get Item Images');

  console.log('Completed data fetching successfully.');
} catch (err) {
  console.error('Failed to fetch all images due to: ', err);
}
