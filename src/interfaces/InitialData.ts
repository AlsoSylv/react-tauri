import { AutoCompleteOption } from './AutoCompleteOption';
import Role from './Role';

interface InitialData {
  languageList: AutoCompleteOption<string>[];
  roleList: Role[];
  rankList: AutoCompleteOption<string>[];
  regionList: AutoCompleteOption<string>[];
  selectedLanguage: string;
}

export default InitialData;
