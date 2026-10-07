const FLOWS = ['legit', 'rogue-redirect', 'replay', 'over-cap', 'kill'] as const;

export function FlowButtons() {
  return (
    <section style={{ border: '1px solid #ccc', borderRadius: 8, padding: 16, marginBottom: 16 }}>
      <h2>Flows</h2>
      {FLOWS.map((f) => (
        <button key={f} style={{ marginRight: 8, marginBottom: 8 }} onClick={() => alert(`${f}: wire to on-chain ix after devnet deploy (Todo 13)`)}>
          {f}
        </button>
      ))}
    </section>
  );
}
