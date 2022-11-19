interface Passive {
  image: string;
  url: string;
}

interface AbilitiesValue {
  image: string;
  order: string[];
  url: string;
}

interface Abilities {
  passive: Passive;
  q: AbilitiesValue;
  w: AbilitiesValue;
  e: AbilitiesValue;
  r: AbilitiesValue;
}

export { Passive, AbilitiesValue, Abilities };
