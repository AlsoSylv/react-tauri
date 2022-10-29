export interface Shard {
  name: string;
  id: number;
  url: string;
  active: boolean;
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

export interface PrimaryRunes {
  slotOne: RuneData[];
  slotTwo: RuneData[];
  slotThree: RuneData[];
  slotFour: RuneData[];
}

export interface SecondaryRunes {
  slotOne: RuneData[];
  slotTwo: RuneData[];
  slotThree: RuneData[];
}

export interface Trees {
  primaryRunes: PrimaryRunes;
  secondaryRunes: SecondaryRunes;
}

export interface ChampionInfo {
  runes: Trees;
  shards: Shards;
  winRate: string;
}

// export default ChampionInfo;
