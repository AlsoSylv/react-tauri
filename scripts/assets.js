/* eslint-disable no-console */
import { createWriteStream } from 'fs';
import { access, mkdir } from 'fs/promises';
import { resolve, join } from 'path';

const rootFolder = resolve('./');

const DDRAGON_URL = 'http://ddragon.leagueoflegends.com';

const version = await (await fetch(`${DDRAGON_URL}/api/versions.json`)).json();
const runePaths = await (await fetch(`${DDRAGON_URL}/cdn/${version[0]}/data/en_US/runesReforged.json`)).json();

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

const getAndSaveImage = async (icon, folder, name) => {
  const imgUrl = `${DDRAGON_URL}/cdn/img/${icon}`;

  const { body } = await fetch(imgUrl);

  await writeFile(folder, `${name}.png`, body);
};

try {
  console.time('getImages');
  const completeRunes = runePaths.flatMap(({ slots, icon: runePathIcon, key: runePathName }) => {
    const runePath = [getAndSaveImage(runePathIcon, `runes/${runePathName}`, runePathName)];

    const runesImages = slots.flatMap(({ runes }) =>
      runes.map(({ icon, key }) => getAndSaveImage(icon, `runes/${runePathName}`, key))
    );

    return runePath.concat(runesImages);
  });

  await Promise.all(completeRunes);
  console.timeEnd('getImages');
  console.log('Completed data fetching successfully.');
} catch (err) {
  console.error('Failed to fetch all images due to: ', err);
}