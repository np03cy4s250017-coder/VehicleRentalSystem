<script lang="ts">
  import '../../app.css';
  import { user, logout } from '$lib/stores/auth';
  import { Shield, LogOut, ChevronLeft } from 'lucide-svelte';

  let { children } = $props();

  function handleLogout() { logout(); window.location.href = '/'; }
</script>

<div class="min-h-screen flex flex-col bg-paper">
  <header class="fixed top-0 left-0 right-0 z-50 bg-paper/90 backdrop-blur-md border-b border-ink/5 h-16">
    <div class="max-w-7xl mx-auto px-6 lg:px-8 h-full flex items-center justify-between">
      <div class="flex items-center gap-4">
        <a href="/" class="flex items-center gap-1.5 text-ink/30 hover:text-ink/60 transition-colors">
          <ChevronLeft class="w-4 h-4" />
          <span class="text-[13px] hidden sm:inline">Back to site</span>
        </a>
        <div class="w-px h-6 bg-ink/10"></div>
        <div class="flex items-center gap-2">
          <span class="font-display text-xl tracking-wider text-ink">YATRA</span>
          <span class="font-display text-xl tracking-wider text-saffron">SATHI</span>
        </div>
        <div class="flex items-center gap-1.5 px-2.5 py-1 bg-crimson/10 rounded-lg">
          <Shield class="w-3.5 h-3.5 text-crimson" />
          <span class="text-[12px] font-semibold tracking-wider uppercase text-crimson">Admin</span>
        </div>
      </div>
      <div class="flex items-center gap-4">
        <span class="text-[14px] text-ink/40">{$user?.name || $user?.phone || 'Admin'}</span>
        <button onclick={handleLogout} class="p-2 rounded-xl text-ink/30 hover:text-ink/60 hover:bg-ink/5 transition-colors" title="Logout">
          <LogOut class="w-4 h-4" />
        </button>
      </div>
    </div>
  </header>
  <main class="flex-1 pt-16">
    {@render children()}
  </main>
</div>
