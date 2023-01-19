import { AutoCompleteOption, ChampionOptions } from './AutoCompleteOption';
import Role from './Role';

interface State {
  rank: string;
  role: string;
  region: string;
  champion: AutoCompleteOption<{ id: number; key: string }> | null;
  roleList: Role[];
  regionList: AutoCompleteOption<string>[];
  championList: ChampionOptions[];
  rankList: AutoCompleteOption<string>[];
  languageList: AutoCompleteOption<string>[];
  selectedLanguage: string;
}

export default State;
