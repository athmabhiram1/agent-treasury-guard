import { useMemo } from 'react';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import { FlowButtons } from './components/FlowButtons';
import { MandateCard } from './components/MandateCard';
import { TxList } from './components/TxList';
import { TxProvider } from './lib/tx';

import '@solana/wallet-adapter-react-ui/styles.css';

export function App() {
  const endpoint = 'https://api.devnet.solana.com';
  const wallets = useMemo(() => [new PhantomWalletAdapter()], []);
  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <TxProvider>
            <main style={{ maxWidth: 720, margin: '0 auto', padding: 24, fontFamily: 'system-ui' }}>
              <h1>Agent Treasury Guard</h1>
              <p>Mandate-bound agent disbursement vault. All policy enforced on-chain.</p>
              <MandateCard />
              <FlowButtons />
              <TxList />
            </main>
          </TxProvider>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
