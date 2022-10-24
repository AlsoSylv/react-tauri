interface ChampionInfo {
  runes: [[{ name: string; url: string; active: boolean }]];
  shards: string[];
  winRate: string;
}

export default ChampionInfo;
