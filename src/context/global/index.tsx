import { createContext, PropsWithChildren, useContext, useMemo, useReducer } from 'react';

import { Context } from 'interfaces';

import ContextReducer from './reducer';
import initialGlobalState from './state';

const GlobalState = createContext<Context>({
  state: initialGlobalState,
  dispatch: () => null,
});

function useGlobalContext() {
  const ctx = useContext(GlobalState);

  if (!ctx) {
    throw new Error('useGlobalContext must be used under GlobalStateProvider');
  }

  return ctx;
}

function GlobalStateProvider({ children }: PropsWithChildren) {
  const [state, dispatch] = useReducer(ContextReducer, initialGlobalState);
  console.log(state);

  const contextValue: Context = useMemo(
    () => ({
      state,
      dispatch,
    }),
    [state]
  );

  return <GlobalState.Provider value={contextValue}>{children}</GlobalState.Provider>;
}

export { useGlobalContext, GlobalStateProvider };
