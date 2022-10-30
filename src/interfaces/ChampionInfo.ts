interface Shard {
  name: string;
  id: number;
  image: string;
  active: boolean;
}

interface Shards {
  rowOne: Shard[];
  rowTwo: Shard[];
  rowThree: Shard[];
}

interface RuneData {
  name: string;
  image: string;
  active: boolean;
}

interface PrimaryRunes {
  slotOne: RuneData[];
  slotTwo: RuneData[];
  slotThree: RuneData[];
  slotFour: RuneData[];
}

interface SecondaryRunes {
  slotOne: RuneData[];
  slotTwo: RuneData[];
  slotThree: RuneData[];
}

interface Trees {
  primaryRunes: PrimaryRunes;
  secondaryRunes: SecondaryRunes;
}

interface ChampionInfo {
  runes: Trees;
  shards: Shards;
  winRate: string;
}

export { Shard, Shards, RuneData, PrimaryRunes, SecondaryRunes, Trees, ChampionInfo };
