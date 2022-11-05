/* eslint-disable no-console */
import { createWriteStream } from 'fs';
import { access, mkdir } from 'fs/promises';
import { join } from 'path';

import { DDRAGON_URL, ROOT_FOLDER } from './constants.js';

const getRandomMs = (minMs, maxMs) =>
  (minMs + ((maxMs - minMs + 1) * crypto.getRandomValues(new Uint32Array(1))[0]) / 2 ** 32) | 0;

const delayedPromise = (promise) =>
  new Promise((r, reject) => {
    setTimeout(async () => {
      promise().then(r).catch(reject);
    }, getRandomMs(0, 1000));
  });

async function pathExists(filePath) {
  try {
    await access(filePath);
    return true;
  } catch {
    return false;
  }
}

const createFilePath = (folderName) => join(ROOT_FOLDER, 'public/', folderName);

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
    const folderDir = createFilePath(folderName);
    const doesPathExist = await pathExists(folderDir);

    if (!doesPathExist) {
      await mkdir(folderDir, { recursive: true });
    }

    const downloadWriteStream = createWriteStream(join(folderDir, fileName));

    data.pipeTo(stream(downloadWriteStream));
  } catch (err) {
    console.log(err);
    Promise.reject(err);
  }
}

const saveImage = async (url, folder, name) => {
  const imgUrl = `${DDRAGON_URL}/cdn/${url}`;

  try {
    const folderDir = createFilePath(folder);
    const fileExists = await pathExists(join(folderDir, name));

    if (fileExists) {
      console.log(`File '${folder}/${name}' already exists on disk, returning...`);
      return Promise.resolve();
    }

    const { body } = await fetch(imgUrl);

    await writeFile(folder, name, body);
    return Promise.resolve();
  } catch (error) {
    console.error('error on: ', imgUrl);
    console.error(error);
    return Promise.reject(error);
  }
};

export { createFilePath, delayedPromise, getRandomMs, pathExists, saveImage };
