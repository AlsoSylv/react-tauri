import fs from 'fs';

let version = await fetch('https://ddragon.leagueoflegends.com/api/versions.json');
version = await version.json();
let runeJson = await fetch(`http://ddragon.leagueoflegends.com/cdn/${version[0]}/data/en_US/runesReforged.json`);
runeJson = await runeJson.text();

fs.writeFile('src/assets/versions.json', runeJson, (err) => {
  if (err) {
    console.log(err);
  }
});
