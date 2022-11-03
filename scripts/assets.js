import fs from 'fs';

let version = await fetch('https://ddragon.leagueoflegends.com/api/versions.json');
version = await version.json();
let runeJson = await fetch(`http://ddragon.leagueoflegends.com/cdn/${version[0]}/data/en_US/runesReforged.json`);
runeJson = await runeJson.json();

for (let y = 0; y < runeJson.length; y += 1) {
  for (let i = 0; i < runeJson[y].slots.length; i += 1) {
    for (let x = 0; x < runeJson[y].slots[i].runes.length; x += 1) {
      console.log(runeJson[y].slots[i].runes[x]);
      const imgUrl = `http://ddragon.leagueoflegends.com/cdn/img/${runeJson[y].slots[i].runes[x].icon}`;
      fetch(imgUrl).then((res) => {
        res.body.pipeThrough(fs.createWriteStream(`src/assets/`));
      });
    }
  }
}

fs.writeFile('src/assets/versions.json', runeJson, (err) => {
  if (err) {
    console.log(err);
  }
});
