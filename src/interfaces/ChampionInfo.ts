export interface Shard {
  name: String;
  id: Number;
  url: String;
  active: Boolean;
}

export interface Shards {
  rowOne: Shard[];
  rowTwo: Shard[];
  rowThree: Shard[];
}

export interface RuneData {
  name: string;
  image: string;
  active: boolean;
}

export interface Trees {
  primaryRunes: RuneData[];
  secondaryRunes: RuneData[];
}

export interface ChampionInfo {
  runes: Trees;
  shards: Shards;
  winRate: string;
}

// export default ChampionInfo;
