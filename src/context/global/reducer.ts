import { State } from 'interfaces';

import { Actions, GlobalActions } from './actions';
import initialGlobalState from './state';

// eslint-disable-next-line default-param-last
const contextReducer = (state: State = initialGlobalState, action: GlobalActions): State => {
  console.log(action);
  switch (action.type) {
    case Actions.UPDATE_CHAMPION: {
      return { ...state, champion: action.payload };
    }
    case Actions.UPDATE_RANK: {
      return { ...state, rank: action.payload };
    }
    case Actions.UPDATE_REGION: {
      return { ...state, region: action.payload };
    }
    case Actions.UPDATE_ROLE: {
      return { ...state, role: action.payload };
    }
    case Actions.SET_INITIAL_DATA: {
      return { ...state, ...action.payload };
    }
    case Actions.SET_SELECTED_LANGUAGE: {
      return { ...state, selectedLanguage: action.payload };
    }
    case Actions.SET_CHAMPIONS_LIST: {
      return { ...state, championList: action.payload };
    }
    default: {
      return state;
    }
  }
};

export default contextReducer;
