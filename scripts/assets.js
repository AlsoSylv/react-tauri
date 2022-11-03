import fs from 'fs';
import { WritableStream } from 'node:stream/web';

let version = await fetch('https://ddragon.leagueoflegends.com/api/versions.json');
version = await version.json();
let runeJson = await fetch(`http://ddragon.leagueoflegends.com/cdn/${version[0]}/data/en_US/runesReforged.json`);
runeJson = await runeJson.json();

const downloadWriteStream = (name) => fs.createWriteStream(`./src/assets/${name}`);

const stream = (name) =>
  new WritableStream({
    write(chunk) {
      downloadWriteStream(name).write(chunk);
    },
  });

for (let y = 0; y < runeJson.length; y += 1) {
  for (let i = 0; i < runeJson[y].slots.length; i += 1) {
    for (let x = 0; x < runeJson[y].slots[i].runes.length; x += 1) {
      const imgUrl = `http://ddragon.leagueoflegends.com/cdn/img/${runeJson[y].slots[i].runes[x].icon}`;
      console.log(imgUrl);
      // eslint-disable-next-line no-await-in-loop
      await fetch(imgUrl).then((res) => {
        res.body.pipeTo(stream(`${runeJson[y].slots[i].runes[x].key}.png`));
      });
    }
  }
}
