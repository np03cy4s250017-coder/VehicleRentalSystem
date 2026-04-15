<script lang="ts">
  import { page } from '$app/stores';
  import { isAuthenticated, isAdmin, isOwner, user, logout, roleLabel } from '$lib/stores/auth';
  import { Menu, X, ChevronDown, User, LayoutDashboard, Shield, LogOut, Car } from 'lucide-svelte';

  let mobileOpen = $state(false);
  let dropdownOpen = $state(false);

  function handleLogout() { logout(); dropdownOpen = false; mobileOpen = false; window.location.href = '/'; }
  function closeMobile() { mobileOpen = false; }
  function toggleDropdown() { dropdownOpen = !dropdownOpen; }
</script>

<svelte:window onclick={() => { if (dropdownOpen) dropdownOpen = false; }} />

<nav class="fixed top-0 left-0 right-0 z-50 bg-paper/90 backdrop-blur-md border-b border-ink/5">
  <div class="max-w-7xl mx-auto px-6 lg:px-8">
    <div class="flex items-center justify-between h-16">
      <a href="/" class="flex items-center gap-1.5">
        <span class="font-display text-[26px] tracking-wide text-ink">YATRA</span>
        <span class="font-display text-[26px] tracking-wide text-saffron">SATHI</span>
      </a>

      <div class="hidden md:flex items-center gap-8">
        <a href="/" class="text-[15px] font-medium transition-colors {$page.url.pathname === '/' ? 'text-ink' : 'text-ink/40 hover:text-ink'}">Home</a>
        {#if !$isOwner}
          <a href="/#vehicles" class="text-[15px] font-medium text-ink/40 hover:text-ink transition-colors">Vehicles</a>
          <a href="/bundles" class="text-[15px] font-medium transition-colors {$page.url.pathname === '/bundles' ? 'text-ink' : 'text-ink/40 hover:text-ink'}">Bundles</a>
          <a href="/drivers" class="text-[15px] font-medium transition-colors {$page.url.pathname === '/drivers' ? 'text-ink' : 'text-ink/40 hover:text-ink'}">Drivers</a>
        {/if}
        {#if $isAuthenticated}
          <a href="/bookings" class="text-[15px] font-medium transition-colors {$page.url.pathname === '/bookings' ? 'text-ink' : 'text-ink/40 hover:text-ink'}">Bookings</a>
          {#if $isOwner}
            <a href="/dashboard" class="text-[15px] font-medium transition-colors {$page.url.pathname === '/dashboard' ? 'text-ink' : 'text-ink/40 hover:text-ink'}">Dashboard</a>
          {/if}
        {/if}
      </div>

      <div class="hidden md:flex items-center">
        {#if $isAuthenticated}
          <div class="relative">
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <button onclick={(e) => { e.stopPropagation(); toggleDropdown(); }}
              class="flex items-center gap-3 px-3 py-2 rounded-xl hover:bg-ink/5 transition-colors">
              <div class="w-9 h-9 rounded-full bg-sage/10 flex items-center justify-center text-sm font-semibold text-sage">
                {($user?.name || $user?.phone || 'U').charAt(0).toUpperCase()}
              </div>
              <div class="text-left">
                <span class="text-[14px] font-medium text-ink block leading-tight">{$user?.name || $user?.phone || 'User'}</span>
                <span class="text-[11px] text-ink/30 leading-tight">{$roleLabel}</span>
              </div>
              <ChevronDown class="w-4 h-4 text-ink/30" />
            </button>

            {#if dropdownOpen}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div onclick={(e) => e.stopPropagation()}
                class="absolute right-0 mt-2 w-52 bg-white rounded-xl shadow-xl shadow-ink/5 border border-ink/5 py-2 z-50">
                <a href="/dashboard" onclick={() => dropdownOpen = false}
                  class="flex items-center gap-3 px-4 py-2.5 text-[14px] text-ink/70 hover:bg-paper transition-colors">
                  <LayoutDashboard class="w-[18px] h-[18px]" /> Dashboard
                </a>
                <a href="/bookings" onclick={() => dropdownOpen = false}
                  class="flex items-center gap-3 px-4 py-2.5 text-[14px] text-ink/70 hover:bg-paper transition-colors">
                  <Car class="w-[18px] h-[18px]" /> My Bookings
                </a>
                {#if $isAdmin}
                  <a href="/admin" onclick={() => dropdownOpen = false}
                    class="flex items-center gap-3 px-4 py-2.5 text-[14px] text-crimson hover:bg-crimson/5 transition-colors">
                    <Shield class="w-[18px] h-[18px]" /> Admin Panel
                  </a>
                {/if}
                <hr class="my-1.5 border-ink/5" />
                <button onclick={handleLogout}
                  class="flex items-center gap-3 w-full px-4 py-2.5 text-[14px] text-crimson/70 hover:bg-crimson/5 transition-colors">
                  <LogOut class="w-[18px] h-[18px]" /> Logout
                </button>
              </div>
            {/if}
          </div>
        {:else}
          <a href="/auth" class="px-5 py-2 bg-ink text-paper text-[14px] font-semibold rounded-xl hover:bg-ink/90 transition-colors">
            Login
          </a>
        {/if}
      </div>

      <button onclick={() => mobileOpen = !mobileOpen} class="md:hidden p-2 rounded-xl hover:bg-ink/5 transition-colors">
        {#if mobileOpen}<X class="w-5 h-5 text-ink" />{:else}<Menu class="w-5 h-5 text-ink" />{/if}
      </button>
    </div>
  </div>

  {#if mobileOpen}
    <div class="md:hidden bg-paper border-t border-ink/5">
      <div class="px-5 py-4 space-y-1">
        <a href="/" onclick={closeMobile} class="block px-4 py-2.5 rounded-xl text-[15px] font-medium text-ink/70 hover:bg-ink/5">Home</a>
        {#if !$isOwner}
          <a href="/#vehicles" onclick={closeMobile} class="block px-4 py-2.5 rounded-xl text-[15px] font-medium text-ink/70 hover:bg-ink/5">Vehicles</a>
          <a href="/bundles" onclick={closeMobile} class="block px-4 py-2.5 rounded-xl text-[15px] font-medium text-ink/70 hover:bg-ink/5">Bundles</a>
          <a href="/drivers" onclick={closeMobile} class="block px-4 py-2.5 rounded-xl text-[15px] font-medium text-ink/70 hover:bg-ink/5">Drivers</a>
        {/if}
        {#if $isAuthenticated}
          <a href="/bookings" onclick={closeMobile} class="block px-4 py-2.5 rounded-xl text-[15px] font-medium text-ink/70 hover:bg-ink/5">Bookings</a>
          <a href="/dashboard" onclick={closeMobile} class="block px-4 py-2.5 rounded-xl text-[15px] font-medium text-ink/70 hover:bg-ink/5">Dashboard</a>
          {#if $isAdmin}
            <a href="/admin" onclick={closeMobile} class="block px-4 py-2.5 rounded-xl text-[15px] font-medium text-crimson hover:bg-crimson/5">Admin Panel</a>
          {/if}
          <button onclick={handleLogout} class="w-full text-left px-4 py-2.5 rounded-xl text-[15px] font-medium text-crimson/70 hover:bg-crimson/5">Logout</button>
        {:else}
          <a href="/auth" onclick={closeMobile} class="block px-4 py-2.5 rounded-xl text-[15px] font-semibold text-center bg-ink text-paper">Login</a>
        {/if}
      </div>
    </div>
  {/if}
</nav>
