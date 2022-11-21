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

export { Shard, Shards };
