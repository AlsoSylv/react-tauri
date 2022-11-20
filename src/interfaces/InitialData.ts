import { AutoCompleteOption } from './AutoCompleteOption';

interface InitialData {
  languageList: AutoCompleteOption<string>[];
  roleList: AutoCompleteOption<string>[];
  rankList: AutoCompleteOption<string>[];
  regionList: AutoCompleteOption<string>[];
  selectedLanguage: string;
}

export default InitialData;
