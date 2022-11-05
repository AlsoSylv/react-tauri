import { resolve } from 'path';

const ROOT_FOLDER = resolve('./');
const DDRAGON_URL = 'https://ddragon.leagueoflegends.com';
const DDRAGON_VERSION = (await (await fetch(`${DDRAGON_URL}/api/versions.json`)).json())[0];

export { ROOT_FOLDER, DDRAGON_URL, DDRAGON_VERSION };
