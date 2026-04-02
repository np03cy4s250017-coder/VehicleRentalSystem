export const ssr = false;

export async function load({ fetch }) {
  try {
    const res = await fetch('http://localhost:3001/api/vehicles');
    if (!res.ok) throw new Error('Failed to fetch');
    const data = await res.json();
    return { vehicles: data.vehicles || data || [] };
  } catch {
    return { vehicles: [] };
  }
}
