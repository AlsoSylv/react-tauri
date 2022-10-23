import { Dispatch } from 'react';

import { GlobalActions } from 'context/global/actions';

import State from './State';

interface Context {
  state: State;
  dispatch: Dispatch<GlobalActions>;
}

export default Context;
