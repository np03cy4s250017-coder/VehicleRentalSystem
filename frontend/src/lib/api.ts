const BASE_URL = 'http://localhost:3001/api';

function getToken(): string | null {
  if (typeof window === 'undefined') return null;
  const raw = localStorage.getItem('yatrasathi_token');
  if (!raw) return null;
  try { return JSON.parse(raw); } catch { return raw; }
}

async function apiFetch(endpoint: string, options: RequestInit = {}): Promise<Response> {
  const token = getToken();
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(options.headers as Record<string, string> || {}),
  };
  if (token) headers['Authorization'] = `Bearer ${token}`;
  return fetch(`${BASE_URL}${endpoint}`, { ...options, headers });
}

// ─── Auth: Login (phone + password) ─────────────────────
export async function loginWithPassword(phone: string, password: string) {
  const res = await apiFetch('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ phone, password }),
  });
  return res.json();
}

// Admin 2FA: verify OTP after password
export async function verifyAdminOtp(phone: string, otp: string) {
  const res = await apiFetch('/auth/login/verify-otp', {
    method: 'POST',
    body: JSON.stringify({ phone, otp }),
  });
  return res.json();
}

// ─── Auth: Register ─────────────────────────────────────
export async function registerAccount(data: { phone: string; password: string; name: string; role: string }) {
  const res = await apiFetch('/auth/register', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  return res.json();
}

export async function verifyRegistration(data: { phone: string; otp: string; name: string; password: string; role: string }) {
  const res = await apiFetch('/auth/register/verify', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  return res.json();
}

// ─── Auth: Password Reset ───────────────────────────────
export async function requestPasswordReset(phone: string) {
  const res = await apiFetch('/auth/reset-password', {
    method: 'POST',
    body: JSON.stringify({ phone }),
  });
  return res.json();
}

export async function verifyPasswordReset(phone: string, otp: string, newPassword: string) {
  const res = await apiFetch('/auth/reset-password/verify', {
    method: 'POST',
    body: JSON.stringify({ phone, otp, new_password: newPassword }),
  });
  return res.json();
}

// ─── Legacy OTP (backward compat) ───────────────────────
export async function sendOtp(phone: string) {
  const res = await apiFetch('/auth/otp/send', { method: 'POST', body: JSON.stringify({ phone }) });
  return res.json();
}

export async function verifyOtp(phone: string, otp: string) {
  const res = await apiFetch('/auth/otp/verify', { method: 'POST', body: JSON.stringify({ phone, otp }) });
  return res.json();
}

// ─── Vehicles ───────────────────────────────────────────
export interface VehicleFilters {
  type?: string; evOnly?: boolean; minPrice?: number; maxPrice?: number; location?: string; sort?: string;
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

export async function searchVehicles(filters: Record<string, any>) {
  const res = await apiFetch('/vehicles/search', { method: 'POST', body: JSON.stringify(filters) });
  return res.json();
}

// ─── Bookings ───────────────────────────────────────────
export async function createBooking(data: {
  vehicleId: string; startDate: string; endDate: string; paymentMethod: string; totalAmount: number;
}) {
  const res = await apiFetch('/bookings', {
    method: 'POST',
    body: JSON.stringify({
      vehicle_id: data.vehicleId,
      start_time: `${data.startDate}T00:00:00`,
      end_time: `${data.endDate}T00:00:00`,
      payment_method: data.paymentMethod,
    }),
  });
  return res.json();
}

export async function getBookings() {
  const res = await apiFetch('/bookings');
  return res.json();
}

// ─── Payments ───────────────────────────────────────────
export async function verifyEsewaPayment(data: { bookingId: string; referenceId: string; productId: string; amount: number }) {
  const res = await apiFetch('/payments/esewa/verify', {
    method: 'POST',
    body: JSON.stringify({ booking_id: data.bookingId, reference_id: data.referenceId, product_id: data.productId, amount: data.amount }),
  });
  return res.json();
}

export async function verifyKhaltiPayment(data: { bookingId: string; token: string; amount: number }) {
  const res = await apiFetch('/payments/khalti/verify', {
    method: 'POST',
    body: JSON.stringify({ booking_id: data.bookingId, token: data.token, amount: data.amount }),
  });
  return res.json();
}

// ─── Admin ──────────────────────────────────────────────
export async function getAdminStats() { return (await apiFetch('/admin/stats')).json(); }
export async function getAdminUsers() { return (await apiFetch('/admin/users')).json(); }
export async function getAdminBookings() { return (await apiFetch('/admin/bookings')).json(); }
export async function getAdminVehicles() { return (await apiFetch('/admin/vehicles')).json(); }

export async function updateBookingStatus(bookingId: string, status: string) {
  const res = await apiFetch(`/bookings/${bookingId}/status`, { method: 'PATCH', body: JSON.stringify({ status }) });
  return res.json();
}

export async function deleteVehicle(vehicleId: string) {
  const res = await apiFetch(`/vehicles/${vehicleId}`, { method: 'DELETE' });
  return res.json();
}

export async function createVehicle(data: Record<string, any>) {
  const res = await apiFetch('/vehicles', { method: 'POST', body: JSON.stringify(data) });
  return res.json();
}
