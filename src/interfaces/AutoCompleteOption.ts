interface AutoCompleteOption {
  label: string;
  value: string;
}

interface ChampionOptions extends AutoCompleteOption {
  url: string;
  localImage: string;
}

export { ChampionOptions, AutoCompleteOption };
