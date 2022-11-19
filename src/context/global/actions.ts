/* eslint-disable no-unused-vars */

import { AutoCompleteOption, ChampionOptions } from 'interfaces';

// eslint-disable-next-line no-shadow
enum Actions {
  UPDATE_ROLE = 'UPDATE_ROLE',
  UPDATE_RANK = 'UPDATE_RANK',
  UPDATE_REGION = 'UPDATE_REGION',
  UPDATE_CHAMPION = 'UPDATE_CHAMPION',
  SET_INITIAL_DATA = 'SET_INITIAL_DATA',
  SET_SELECTED_LANGUAGE = 'SET_SELECTED_LANGUAGE',
  SET_CHAMPIONS_LIST = 'SET_CHAMPIONS_LIST',
}

type GlobalActions =
  | {
      type: Actions.UPDATE_CHAMPION;
      payload: AutoCompleteOption<{ id: number; key: string }> | null;
    }
  | {
      type: Actions.UPDATE_RANK;
      payload: string;
    }
  | {
      type: Actions.UPDATE_REGION;
      payload: string;
    }
  | {
      type: Actions.UPDATE_ROLE;
      payload: string;
    }
  | {
      type: Actions.SET_INITIAL_DATA;
      payload: {
        roleList: AutoCompleteOption<string>[];
        regionList: AutoCompleteOption<string>[];
        rankList: AutoCompleteOption<string>[];
        languageList: AutoCompleteOption<string>[];
      };
    }
  | {
      type: Actions.SET_SELECTED_LANGUAGE;
      payload: string;
    }
  | {
      type: Actions.SET_CHAMPIONS_LIST;
      payload: ChampionOptions[];
    };

export { Actions, GlobalActions };
