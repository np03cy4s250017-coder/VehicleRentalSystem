import { writable, derived } from 'svelte/store';

export interface User {
  id: string;
  phone: string;
  name?: string;
  role: 'consumer' | 'owner' | 'driver' | 'admin';
  avatar_url?: string;
}

function createPersistedStore<T>(key: string, initial: T) {
  const stored = typeof window !== 'undefined' ? localStorage.getItem(key) : null;
  const data = stored ? JSON.parse(stored) as T : initial;
  const store = writable<T>(data);

  store.subscribe((value) => {
    if (typeof window !== 'undefined') {
      if (value === null || value === undefined) {
        localStorage.removeItem(key);
      } else {
        localStorage.setItem(key, JSON.stringify(value));
      }
    }
  });

  return store;
}

export const user = createPersistedStore<User | null>('yatrasathi_user', null);
export const token = createPersistedStore<string | null>('yatrasathi_token', null);

export const isAuthenticated = derived([user, token], ([$user, $token]) => !!$user && !!$token);
export const isAdmin = derived(user, ($user) => $user?.role === 'admin');
export const isOwner = derived(user, ($user) => $user?.role === 'owner');
export const isDriver = derived(user, ($user) => $user?.role === 'driver');
export const isConsumer = derived(user, ($user) => $user?.role === 'consumer');

export const roleLabel = derived(user, ($user) => {
  const labels: Record<string, string> = {
    consumer: 'Consumer',
    owner: 'Vehicle Owner',
    driver: 'Driver',
    admin: 'Admin',
    renter: 'Consumer',
  };
  return labels[$user?.role || 'consumer'] || 'User';
});

export function login(newToken: string, newUser: User) {
  token.set(newToken);
  user.set(newUser);
}

export function logout() {
  token.set(null);
  user.set(null);
}
