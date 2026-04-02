const BASE_URL = 'http://localhost:3001/api';

function getToken(): string | null {
  if (typeof window === 'undefined') return null;
  const raw = localStorage.getItem('yatrasathi_token');
  if (!raw) return null;
  try {
    return JSON.parse(raw);
  } catch {
    return raw;
  }
}

async function apiFetch(endpoint: string, options: RequestInit = {}): Promise<Response> {
  const token = getToken();
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(options.headers as Record<string, string> || {}),
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const res = await fetch(`${BASE_URL}${endpoint}`, {
    ...options,
    headers,
  });

  return res;
}

export async function sendOtp(phone: string) {
  const res = await apiFetch('/auth/otp/send', {
    method: 'POST',
    body: JSON.stringify({ phone }),
  });
  return res.json();
}

export async function verifyOtp(phone: string, otp: string) {
  const res = await apiFetch('/auth/otp/verify', {
    method: 'POST',
    body: JSON.stringify({ phone, otp }),
  });
  return res.json();
}

export interface VehicleFilters {
  type?: string;
  evOnly?: boolean;
  minPrice?: number;
  maxPrice?: number;
  location?: string;
  sort?: string;
}

export async function getVehicles(filters?: VehicleFilters) {
  const params = new URLSearchParams();
  if (filters) {
    if (filters.type && filters.type !== 'all') params.set('type', filters.type);
    if (filters.evOnly) params.set('evOnly', 'true');
    if (filters.minPrice) params.set('minPrice', String(filters.minPrice));
    if (filters.maxPrice) params.set('maxPrice', String(filters.maxPrice));
    if (filters.location) params.set('location', filters.location);
    if (filters.sort) params.set('sort', filters.sort);
  }
  const query = params.toString();
  const res = await apiFetch(`/vehicles${query ? `?${query}` : ''}`);
  return res.json();
}

export async function getVehicle(id: string) {
  const res = await apiFetch(`/vehicles/${id}`);
  return res.json();
}

export async function createBooking(data: {
  vehicleId: string;
  startDate: string;
  endDate: string;
  paymentMethod: string;
  totalAmount: number;
}) {
  const res = await apiFetch('/bookings', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  return res.json();
}

export async function getBookings() {
  const res = await apiFetch('/bookings');
  return res.json();
}

export async function getAdminStats() {
  const res = await apiFetch('/admin/stats');
  return res.json();
}

export async function getAdminUsers() {
  const res = await apiFetch('/admin/users');
  return res.json();
}

export async function getAdminBookings() {
  const res = await apiFetch('/admin/bookings');
  return res.json();
}

export async function getAdminVehicles() {
  const res = await apiFetch('/admin/vehicles');
  return res.json();
}
