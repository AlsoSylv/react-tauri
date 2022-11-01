interface AutoCompleteOption {
  label: string;
  value: string;
}

interface ChampionOptions extends AutoCompleteOption {
  url: string;
}

export { ChampionOptions, AutoCompleteOption };
