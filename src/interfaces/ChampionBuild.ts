import type { Abilities } from './Abilities';
import type { Items } from './Items';
import type { Runes } from './Runes';
import type { Shards } from './Shards';

interface ChampionBuild {
  runes: Runes;
  items: Items;
  abilities: Abilities;
  shards: Shards;
}

export default ChampionBuild;
