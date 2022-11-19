interface ItemValues {
  name: string;
  cost: string;
  description: string;
  localImage: string;
  url: string;
}

interface Items {
  start: ItemValues[];
  core: ItemValues[];
  fourth: ItemValues[];
  fifth: ItemValues[];
  sixth: ItemValues[];
}

export { ItemValues, Items };
