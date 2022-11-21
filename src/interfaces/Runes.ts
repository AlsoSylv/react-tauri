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

interface Runes {
  primaryRunes: PrimaryRunes;
  secondaryRunes: SecondaryRunes;
}

export { RuneData, PrimaryRunes, SecondaryRunes, Runes };
