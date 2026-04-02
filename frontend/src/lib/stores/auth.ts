import { writable, derived } from 'svelte/store';

export interface User {
  id: string;
  phone: string;
  name?: string;
  role: 'renter' | 'owner' | 'driver' | 'admin';
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

export const isAuthenticated = derived([user, token], ([$user, $token]) => {
  return !!$user && !!$token;
});

export const isAdmin = derived(user, ($user) => {
  return $user?.role === 'admin';
});

export const isOwner = derived(user, ($user) => {
  return $user?.role === 'owner';
});

export function login(newToken: string, newUser: User) {
  token.set(newToken);
  user.set(newUser);
}

export function logout() {
  token.set(null);
  user.set(null);
}
