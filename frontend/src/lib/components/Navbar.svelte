<script lang="ts">
  import { page } from '$app/stores';
  import { isAuthenticated, isAdmin, user, logout } from '$lib/stores/auth';
  import { Menu, X, ChevronDown, User, LayoutDashboard, Shield, LogOut, Zap } from 'lucide-svelte';

  let mobileOpen = $state(false);
  let dropdownOpen = $state(false);

  function handleLogout() {
    logout();
    dropdownOpen = false;
    mobileOpen = false;
    window.location.href = '/';
  }

  function closeMobile() {
    mobileOpen = false;
  }

  function toggleDropdown() {
    dropdownOpen = !dropdownOpen;
  }

  function closeDropdown() {
    dropdownOpen = false;
  }
</script>

<svelte:window onclick={() => { if (dropdownOpen) dropdownOpen = false; }} />

<nav class="fixed top-0 left-0 right-0 z-50 bg-white/95 backdrop-blur-md shadow-sm border-b border-slate-100">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex items-center justify-between h-16">
      <!-- Logo -->
      <a href="/" class="flex items-center gap-2 group">
        <span class="text-2xl">🏔️</span>
        <div class="flex items-center gap-1">
          <span class="text-xl font-bold text-emerald-700 group-hover:text-emerald-600 transition-colors">Yatra</span>
          <span class="text-xl font-bold text-amber-600">Sathi</span>
        </div>
        <Zap class="w-4 h-4 text-sky-500" />
      </a>

      <!-- Desktop Navigation -->
      <div class="hidden md:flex items-center gap-8">
        <a
          href="/"
          class="text-sm font-medium transition-colors {$page.url.pathname === '/' ? 'text-emerald-700' : 'text-slate-600 hover:text-emerald-700'}"
        >
          Home
        </a>
        <a
          href="/#vehicles"
          class="text-sm font-medium text-slate-600 hover:text-emerald-700 transition-colors"
        >
          Vehicles
        </a>
        {#if $isAuthenticated}
          <a
            href="/bookings"
            class="text-sm font-medium transition-colors {$page.url.pathname === '/bookings' ? 'text-emerald-700' : 'text-slate-600 hover:text-emerald-700'}"
          >
            My Bookings
          </a>
        {/if}
      </div>

      <!-- Desktop Right -->
      <div class="hidden md:flex items-center gap-4">
        {#if $isAuthenticated}
          <div class="relative">
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <button
              onclick={(e) => { e.stopPropagation(); toggleDropdown(); }}
              class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-slate-100 transition-colors"
            >
              <div class="w-8 h-8 rounded-full bg-emerald-100 flex items-center justify-center">
                <User class="w-4 h-4 text-emerald-700" />
              </div>
              <span class="text-sm font-medium text-slate-700">
                {$user?.name || $user?.phone || 'User'}
              </span>
              <ChevronDown class="w-4 h-4 text-slate-400" />
            </button>

            {#if dropdownOpen}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div
                onclick={(e) => e.stopPropagation()}
                class="absolute right-0 mt-2 w-48 bg-white rounded-xl shadow-lg border border-slate-100 py-2 z-50"
              >
                <a
                  href="/dashboard"
                  onclick={closeDropdown}
                  class="flex items-center gap-3 px-4 py-2 text-sm text-slate-700 hover:bg-slate-50 transition-colors"
                >
                  <LayoutDashboard class="w-4 h-4" />
                  Dashboard
                </a>
                {#if $isAdmin}
                  <a
                    href="/admin"
                    onclick={closeDropdown}
                    class="flex items-center gap-3 px-4 py-2 text-sm text-slate-700 hover:bg-slate-50 transition-colors"
                  >
                    <Shield class="w-4 h-4" />
                    Admin Panel
                  </a>
                {/if}
                <hr class="my-1 border-slate-100" />
                <button
                  onclick={handleLogout}
                  class="flex items-center gap-3 w-full px-4 py-2 text-sm text-red-600 hover:bg-red-50 transition-colors"
                >
                  <LogOut class="w-4 h-4" />
                  Logout
                </button>
              </div>
            {/if}
          </div>
        {:else}
          <a
            href="/auth"
            class="px-5 py-2 bg-emerald-600 text-white text-sm font-medium rounded-lg hover:bg-emerald-700 transition-colors shadow-sm"
          >
            Login
          </a>
        {/if}
      </div>

      <!-- Mobile Menu Button -->
      <button
        onclick={() => mobileOpen = !mobileOpen}
        class="md:hidden p-2 rounded-lg hover:bg-slate-100 transition-colors"
      >
        {#if mobileOpen}
          <X class="w-6 h-6 text-slate-700" />
        {:else}
          <Menu class="w-6 h-6 text-slate-700" />
        {/if}
      </button>
    </div>
  </div>

  <!-- Mobile Menu -->
  {#if mobileOpen}
    <div class="md:hidden bg-white border-t border-slate-100 shadow-lg">
      <div class="px-4 py-4 space-y-2">
        <a
          href="/"
          onclick={closeMobile}
          class="block px-4 py-3 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-50 transition-colors"
        >
          Home
        </a>
        <a
          href="/#vehicles"
          onclick={closeMobile}
          class="block px-4 py-3 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-50 transition-colors"
        >
          Vehicles
        </a>
        {#if $isAuthenticated}
          <a
            href="/bookings"
            onclick={closeMobile}
            class="block px-4 py-3 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-50 transition-colors"
          >
            My Bookings
          </a>
          <a
            href="/dashboard"
            onclick={closeMobile}
            class="block px-4 py-3 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-50 transition-colors"
          >
            Dashboard
          </a>
          {#if $isAdmin}
            <a
              href="/admin"
              onclick={closeMobile}
              class="block px-4 py-3 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-50 transition-colors"
            >
              Admin Panel
            </a>
          {/if}
          <button
            onclick={handleLogout}
            class="w-full text-left px-4 py-3 rounded-lg text-sm font-medium text-red-600 hover:bg-red-50 transition-colors"
          >
            Logout
          </button>
        {:else}
          <a
            href="/auth"
            onclick={closeMobile}
            class="block px-4 py-3 rounded-lg text-sm font-medium text-center bg-emerald-600 text-white hover:bg-emerald-700 transition-colors"
          >
            Login
          </a>
        {/if}
      </div>
    </div>
  {/if}
</nav>
