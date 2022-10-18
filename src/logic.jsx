import { Combobox, Popover } from "@headlessui/react";
import { invoke } from "@tauri-apps/api";
import { React, useState, useEffect } from "react";

var exportedRank = "platinum_plus";
var exportedRole = "none";
var exportedRegion = "world";
var exportedChampion = '';


export function Selects() {
  const element = (
    <div id="drop-downs">
    <ChampionOptions />
      <RoleMenu />
      <br></br>
      <RegionMenu />
      <br></br>
      <RankMenu />
      <br></br>
    </div>
  );
  console.log(exportedRank)
  return element;
}


export function Page() {
  const [page = [["bullshit"]], setPage] = useState([[null, null, null, null], [null, null]])
  
  const element = (
    <div>
      <button onClick={() => {
          invoke("rune_names", {
    name: exportedChampion,
    role: exportedRole,
    rank: exportedRank,
    region: exportedRegion
  }).then((runes) => {
    console.log(runes)
    setPage(runes)
    })
      }
        }>Click Me</button>
      <text> <br></br>
      {page[0][0]} <br></br>
      {page[0][1]} <br></br>
      {page[0][2]} <br></br>
      {page[0][3]} </text>
    </div>
  )
  return element
}


//This whole section should be auto generated somehow!
function RoleMenu() {
  const [role = "", setRole] = useState()
  const element = (
    <select id="roles"
    defaultValue="none"
    onChange={(e) => {
      setRole(e.target.value);
      exportedRole = e.target.value;
      }}>
      <option value="none" disabled>None</option>
      <option value="top">Top</option>
      <option value="jungle">Jungle</option>
      <option value="mid">Mid</option>
      <option value="adc">ADC</option>
      <option value="support">Support</option>
    </select>
  )
  console.log(role);
  return element;
}


function RankMenu() {
  const [rank = "platinum_plus", setRank] = useState();
  const element = (
    <select id="rank"
    defaultValue="platinum_plus"
    onChange={(e) => {
      setRank(e.target.value);
      exportedRank = e.target.value;
      }}>
      <option value="challenger">Challenger</option>
      <option value="grandmaster">Grandmaster</option>
      <option value="master">Master</option>
      <option value="diamond">Diamond</option>
      <option value="platinum">Platinum</option>
      <option value="gold">Gold</option>
      <option value="silver">Silver</option>
      <option value="bronze">Bronze</option>
      <option value="iron">Iron</option>
      <option value="overall">All Ranks</option>
      <option value="master_plus">Master +</option>
      <option value="diamond_plus">Diamond +</option>
      <option value="diamond_2_plus0">Diamond 2 +</option>
      <option value="platinum_plus">Platinum +</option>
    </select>
  );
  console.log(rank);
  return element;
}


function RegionMenu() {
  const [region = "world", setRegion] = useState()
  const element = (
    <select id="region"
    defaultValue="world"
    onChange={(e) => {
      setRegion(e.target.value);
      exportedRegion = e.target.value;
      }}>
      <option value="world">World</option>
      <option value="na1">North America</option>
      <option value="euw1">EU West</option>
      <option value="kr">Korea</option>
      <option value="br1">Brazil</option>
      <option value="eun1">EU North</option>
      <option value="jp1">Japan</option>
      <option value="la1">LA North</option>
      <option value="la2">LA South</option>
      <option value="oc1">OCE</option>
      <option value="ru">Russia</option>
      <option value="tr1">Turkey</option>
    </select>
  );
  console.log(region);
  return element;
}


function ChampionOptions() {

  const [champions, setChampions] = useState([null]);
  const [selectedOptions, setSelectedOptions] = useState(champions[0])
  const [query, setQeury] = useState('')

  useEffect(() => {
    invoke("champion_names").then((names) => {
      for (let y = 0; y < names.length; y++) {
        names[y] = names[y].replace(/['"]+/g, '')
      }
      setChampions(names)
    });
  }, [])

  if (champions === null) {
    return <p>loading...</p>
  } else {

  const filteredChampions =
  query === ''
    ? champions
    : champions.filter((champ) => {
    return champ.toLowerCase().includes(query.toLowerCase())
  })

  let topFiveChmapions = [];
  for (let y = 0; y < 5; y++) {
    if (filteredChampions[y] !== undefined) {
      topFiveChmapions.push(filteredChampions[y])
    }
  }

  const element = (
      <div id="champion-popup">
        <Combobox value={selectedOptions} onChange={setSelectedOptions}>
          <Combobox.Input className="champion-popup" onChange={(event) => {
            setQeury(event.target.value)
            exportedChampion = event.target.value
            console.log(event.target.value)
          }} />
          <Combobox.Options>
            {topFiveChmapions.map((champ) => (
              <Combobox.Option key={champ} value={champ} {...exportedChampion = champ}>
                {champ}
              </Combobox.Option>
            ))}
          </Combobox.Options>
        </Combobox>
      </div>
  );
    return element;
  }
}