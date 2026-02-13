export function parseReserve(raw: any) {
  return {
    total: raw?.lamports ?? 0
  };
}
