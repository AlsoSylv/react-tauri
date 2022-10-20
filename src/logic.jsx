import { Combobox } from "@headlessui/react";
import { invoke } from "@tauri-apps/api";
import { React, useState, useEffect } from "react";

const exported = {
  rank: 'platinum_plus',
  role: 'none',
  region: 'world',
  champion: 'Ashe ',
}

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
  return element;
}

export function Page() {
  const [page, setPage] = useState([[null, null, null, null], [null, null]])
  const [champ, setChamp] = useState()

  /*if (page === undefined) {
    return (
      <div id="get-runes">
        <button onClick={() => {
          runes().then((result) => setPage(result))
        }}>Click Me</button>
        <div>
          loading....
        </div>
      </div>
    )
  }*/
  
  const element = (
    <div id="get-runes">
      <button onClick={() => {
        runes().then((runePage) => {
          setPage(runePage)
        })
      }}>Click Me</button>
      <div>
        {page[0][0]} <br></br>
        {page[0][1]} <br></br>
        {page[0][2]} <br></br>
        {page[0][3]} 
      </div>
    </div>
  )
  return element
}


async function runes() {
  var runePage;
  if (exported.champion === undefined || '' && exported.role === 'none') {
    return [["Please Enter A Champion Name And Select A Role", null, null, null], [null, null]]
  } else if (exported.champion === undefined || '') {
    return [["Please Enter A Champion Name", null, null, null], [null, null]]
  } else if (exported.role === 'none') {
    return [["Please Select a Role", null, null, null], [null, null]]
  } else {
    await invoke("rune_names", {
      name: exported.champion,
      role: exported.role,
      rank: exported.rank,
      region: exported.region
    }).then((runes) => {
      console.log(runes)
      runePage = runes
    }).catch(() => {
      runePage = [["No Data Exists!", null, null, null], [null, null]]
    })
  }
  return runePage
}

//This whole section should be auto generated somehow!
function RoleMenu() {
  const [role = "", setRole] = useState()
  const element = (
    <select id="roles"
    defaultValue="none"
    onChange={(e) => {
      setRole(e.target.value);
      exported.role = e.target.value;
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
      exported.rank = e.target.value;
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
      exported.region = e.target.value;
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

  const [champions, setChampions] = useState([]);
  const [selectedChampion, setSelectedOptions] = useState()
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
    : champions.filter((champs) => {
    return champs.toLowerCase().includes(query.toLowerCase())
  })

  let topFiveChmapions = [];
  for (let y = 0; y < 5; y++) {
    if (filteredChampions[y] !== undefined) {
      topFiveChmapions.push(filteredChampions[y])
    }
  }
  
  console.log(topFiveChmapions)

  const element = (
      <div id="champion-popup">
        <Combobox as="div" value={selectedChampion} onChange={() => {
          setSelectedOptions()
          //console.log(selectedChampion)
          }}>
          <Combobox.Input className="champion-popup" displayValue={(champ) => {exported.champion = champ; return champ}} onChange={(event) => {
            setQeury(event.target.value)
          }} />
          <Combobox.Options>
            {topFiveChmapions.map((champ) => (
              <Combobox.Option key={champ} value={champ}>
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