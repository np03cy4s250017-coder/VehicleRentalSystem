<script lang="ts">
  import { onMount } from 'svelte';
  import { getBundles } from '$lib/api';
  import { MapPin, Clock, Loader2, UserCheck, FileCheck } from 'lucide-svelte';

  let bundles = $state<any[]>([]);
  let loading = $state(true);
  let error = $state('');

  onMount(async () => {
    try {
      const res = await getBundles();
      bundles = res.bundles || [];
    } catch {
      error = 'Failed to load tour bundles';
    } finally {
      loading = false;
    }
  });
</script>

<svelte:head><title>Tour Bundles · YatraSathi</title></svelte:head>

<section class="pt-24 pb-20 bg-paper min-h-screen">
  <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
  <h1 class="font-display text-4xl text-ink tracking-wide mb-2">Tour Bundles</h1>
  <p class="text-[15px] text-ink/40 mb-8">Curated trips with vehicle, guide and permits bundled into one booking.</p>

  {#if loading}
    <div class="flex justify-center py-20"><Loader2 class="w-8 h-8 animate-spin text-gray-400" /></div>
  {:else if error}
    <p class="text-red-600">{error}</p>
  {:else if bundles.length === 0}
    <p class="text-gray-500">No bundles available yet.</p>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each bundles as b (b.id)}
        <article class="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden hover:shadow-md transition">
          {#if b.image_url}
            <img src={b.image_url} alt={b.name} class="w-full h-44 object-cover" />
          {/if}
          <div class="p-5 space-y-3">
            <h2 class="text-xl font-semibold">{b.name}</h2>
            <div class="flex items-center gap-3 text-sm text-gray-600">
              <span class="inline-flex items-center gap-1"><MapPin class="w-4 h-4" />{b.destination}</span>
              <span class="inline-flex items-center gap-1"><Clock class="w-4 h-4" />{b.duration_days} day{b.duration_days > 1 ? 's' : ''}</span>
            </div>
            <p class="text-sm text-gray-700 line-clamp-3">{b.description}</p>
            <div class="flex flex-wrap gap-2 text-xs">
              <span class="px-2 py-1 rounded-full bg-gray-100 capitalize">{b.recommended_vehicle}</span>
              {#if b.includes_guide}
                <span class="px-2 py-1 rounded-full bg-emerald-50 text-emerald-700 inline-flex items-center gap-1"><UserCheck class="w-3 h-3" />Guide</span>
              {/if}
              {#if b.includes_permit}
                <span class="px-2 py-1 rounded-full bg-amber-50 text-amber-700 inline-flex items-center gap-1"><FileCheck class="w-3 h-3" />Permits</span>
              {/if}
            </div>
            <div class="pt-2 flex items-center justify-between">
              <span class="text-lg font-bold">Rs. {b.estimated_price.toLocaleString()}</span>
              <span class="text-xs text-gray-500">from</span>
            </div>
          </div>
        </article>
      {/each}
    </div>
  {/if}
  </div>
</section>
