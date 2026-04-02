export const ssr = false;

export async function load({ params, fetch }) {
  try {
    const res = await fetch(`http://localhost:3001/api/vehicles/${params.id}`);
    if (!res.ok) throw new Error('Failed to fetch');
    const data = await res.json();
    return { vehicle: data.vehicle || data || null };
  } catch {
    return { vehicle: null };
  }
}
