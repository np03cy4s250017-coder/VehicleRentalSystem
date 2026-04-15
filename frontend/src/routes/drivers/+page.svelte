<script lang="ts">
  import { onMount } from 'svelte';
  import { getDrivers } from '$lib/api';
  import { Loader2, BadgeCheck, Phone, Languages, Route } from 'lucide-svelte';

  let drivers = $state<any[]>([]);
  let loading = $state(true);
  let error = $state('');

  onMount(async () => {
    try {
      const res = await getDrivers();
      drivers = res.drivers || [];
    } catch {
      error = 'Failed to load drivers';
    } finally {
      loading = false;
    }
  });
</script>

<svelte:head><title>Verified Drivers · YatraSathi</title></svelte:head>

<section class="pt-24 pb-20 bg-paper min-h-screen">
  <div class="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
  <h1 class="font-display text-4xl text-ink tracking-wide mb-2">Verified Drivers</h1>
  <p class="text-[15px] text-ink/40 mb-8">Background-checked drivers with route experience across Nepal.</p>

  {#if loading}
    <div class="flex justify-center py-20"><Loader2 class="w-8 h-8 animate-spin text-gray-400" /></div>
  {:else if error}
    <p class="text-red-600">{error}</p>
  {:else if drivers.length === 0}
    <p class="text-gray-500">No verified drivers yet.</p>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
      {#each drivers as d (d.user_id)}
        <article class="bg-white rounded-2xl shadow-sm border border-gray-100 p-5 space-y-3">
          <div class="flex items-center gap-4">
            <div class="w-14 h-14 rounded-full bg-gradient-to-br from-sky-400 to-indigo-500 text-white flex items-center justify-center text-xl font-bold">
              {d.name?.[0] ?? '?'}
            </div>
            <div class="flex-1">
              <h2 class="text-lg font-semibold flex items-center gap-2">
                {d.name ?? 'Driver'}
                {#if d.verified}<BadgeCheck class="w-5 h-5 text-emerald-600" />{/if}
              </h2>
              <p class="text-sm text-gray-500">{d.years_experience} yrs experience · License {d.license_no}</p>
            </div>
          </div>
          {#if d.bio}
            <p class="text-sm text-gray-700">{d.bio}</p>
          {/if}
          {#if d.languages?.length}
            <div class="flex items-center gap-2 text-xs text-gray-600 flex-wrap">
              <Languages class="w-4 h-4" />
              {#each d.languages as lang}
                <span class="px-2 py-0.5 rounded-full bg-gray-100">{lang}</span>
              {/each}
            </div>
          {/if}
          {#if d.specialties?.length}
            <div class="flex items-center gap-2 text-xs text-gray-600 flex-wrap">
              <Route class="w-4 h-4" />
              {#each d.specialties as sp}
                <span class="px-2 py-0.5 rounded-full bg-indigo-50 text-indigo-700">{sp}</span>
              {/each}
            </div>
          {/if}
          {#if d.phone}
            <a href={`tel:${d.phone}`} class="inline-flex items-center gap-1 text-sm text-sky-600 hover:underline">
              <Phone class="w-4 h-4" />{d.phone}
            </a>
          {/if}
        </article>
      {/each}
    </div>
  {/if}
  </div>
</section>
