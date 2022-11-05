interface Shard {
  name: string;
  id: number;
  image: string;
  active: boolean;
  localImage: string;
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
  localImage: string;
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

interface RuneTrees {
  primaryRunes: PrimaryRunes;
  secondaryRunes: SecondaryRunes;
}

interface ChampionInfo {
  url: string;
  winRate: string;
  pickRate: string;
  banRate: string;
}
interface CompleteChampionInfo extends ChampionInfo {
  runes: RuneTrees;
  shards: Shards;
}

export { Shard, Shards, RuneData, PrimaryRunes, SecondaryRunes, RuneTrees, ChampionInfo, CompleteChampionInfo };
