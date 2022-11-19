interface AutoCompleteOption<T> {
  label: string;
  value: T;
}

interface ChampionOptions extends AutoCompleteOption<{ id: number; key: string }> {
  url: string;
  localImage: string;
}

export { ChampionOptions, AutoCompleteOption };
