import React, { createContext, PropsWithChildren, useContext, useMemo, useReducer } from 'react';

import { Context } from 'interfaces';

import ContextReducer from './reducer';
import initialGlobalState from './state';

const GlobalState = createContext<Context>({
  state: initialGlobalState,
  setState: () => null,
});

function useGlobalContext() {
  const ctx = useContext(GlobalState);

  if (!ctx) {
    throw new Error('useGlobalContext must be used under GlobalStateProvider');
  }

  return ctx;
}

function GlobalStateProvider({ children }: PropsWithChildren) {
  const [state, setState] = useReducer(ContextReducer, initialGlobalState);

  const contextValue: Context = useMemo(
    () => ({
      state,
      setState,
    }),
    [state]
  );

  return <GlobalState.Provider value={contextValue}>{children}</GlobalState.Provider>;
}

export { useGlobalContext, GlobalStateProvider };
