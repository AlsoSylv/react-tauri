interface RuneData {
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
  shards: string[];
  winRate: string;
}

// export default ChampionInfo;
