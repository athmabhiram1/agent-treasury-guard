import { useTx } from '../lib/tx';

export function TxList() {
  const { txs } = useTx();
  return (
    <section style={{ border: '1px solid #ccc', borderRadius: 8, padding: 16 }}>
      <h2>Transactions</h2>
      {txs.length === 0 ? <p>No transactions yet.</p> : (
        <ul>
          {txs.map((t) => (
            <li key={t}><a href={t} target="_blank" rel="noreferrer">{t}</a></li>
          ))}
        </ul>
      )}
    </section>
  );
}
