import { createContext, useContext, useState, type ReactNode } from 'react';

const TxCtx = createContext<{ txs: string[]; push: (u: string) => void }>({
  txs: [],
  push: () => {},
});

export function TxProvider({ children }: { children: ReactNode }) {
  const [txs, setTxs] = useState<string[]>([]);
  return <TxCtx.Provider value={{ txs, push: (u) => setTxs((t) => [u, ...t]) }}>{children}</TxCtx.Provider>;
}

export const useTx = () => useContext(TxCtx);
