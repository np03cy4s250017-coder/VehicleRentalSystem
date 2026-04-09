<script lang="ts">
  import HeroSection from '$lib/components/HeroSection.svelte';
  import SearchFilters from '$lib/components/SearchFilters.svelte';
  import VehicleCard from '$lib/components/VehicleCard.svelte';
  import { Car, ArrowRight } from 'lucide-svelte';

  let { data } = $props();

  let vehicleType = $state('all');
  let evOnly = $state(false);
  let sortBy = $state('');

  let filteredVehicles = $derived.by(() => {
    let result = [...(data.vehicles || [])];
    if (vehicleType !== 'all') result = result.filter((v: any) => v.type === vehicleType);
    if (evOnly) result = result.filter((v: any) => v.is_ev);
    if (sortBy === 'price-asc') result.sort((a: any, b: any) => (a.daily_rate || 0) - (b.daily_rate || 0));
    else if (sortBy === 'price-desc') result.sort((a: any, b: any) => (b.daily_rate || 0) - (a.daily_rate || 0));
    else if (sortBy === 'newest') result.sort((a: any, b: any) => new Date(b.created_at || 0).getTime() - new Date(a.created_at || 0).getTime());
    return result;
  });

  function onFilterChange() {}

  function onHeroSearch(filters: { type: string; location: string }) {
    if (filters.type) vehicleType = filters.type;
    // location filtering can be added when backend supports it
  }
</script>

<svelte:head>
  <title>YatraSathi - Nepal's EV Rental Marketplace</title>
</svelte:head>

<HeroSection onSearch={onHeroSearch} />

<section id="vehicles" class="py-20 bg-paper">
  <div class="max-w-7xl mx-auto px-6 lg:px-8">
    <div class="mb-10">
      <h2 class="font-display text-4xl md:text-5xl text-ink tracking-wide">AVAILABLE VEHICLES</h2>
      <p class="mt-3 text-[16px] text-ink/35">Curated EVs and eco-friendly vehicles across Kathmandu Valley</p>
    </div>

    <SearchFilters bind:vehicleType bind:evOnly bind:sortBy onchange={onFilterChange} />

    {#if filteredVehicles.length > 0}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each filteredVehicles as vehicle (vehicle.id)}
          <VehicleCard {vehicle} />
        {/each}
      </div>
    {:else if data.vehicles?.length === 0}
      <div class="text-center py-20">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-ink/5 rounded-2xl mb-5">
          <Car class="w-8 h-8 text-ink/15" />
        </div>
        <h3 class="text-xl font-semibold text-ink mb-2">No Vehicles Available</h3>
        <p class="text-[15px] text-ink/35">We're adding new vehicles. Check back soon!</p>
      </div>
    {:else}
      <div class="text-center py-20">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-ink/5 rounded-2xl mb-5">
          <Car class="w-8 h-8 text-ink/15" />
        </div>
        <h3 class="text-xl font-semibold text-ink mb-2">No Vehicles Found</h3>
        <p class="text-[15px] text-ink/35 mb-5">Try adjusting your filters.</p>
        <button onclick={() => { vehicleType = 'all'; evOnly = false; sortBy = ''; }}
          class="px-6 py-2.5 bg-sage text-white text-[14px] font-semibold rounded-xl hover:bg-sage-600 transition-colors">
          Clear Filters
        </button>
      </div>
    {/if}
  </div>
</section>

<section class="py-20 bg-ink">
  <div class="max-w-4xl mx-auto px-6 lg:px-8 text-center">
    <h2 class="font-display text-4xl md:text-5xl text-paper tracking-wide mb-4">OWN AN EV? LIST IT HERE</h2>
    <p class="text-paper/35 text-[16px] max-w-xl mx-auto mb-8">
      Earn money by sharing your electric vehicle with riders across Kathmandu.
    </p>
    <a href="/auth"
      class="inline-flex items-center gap-2 px-8 py-3 bg-saffron text-ink font-semibold text-[15px] rounded-xl hover:bg-saffron-300 transition-colors">
      Get Started <ArrowRight class="w-[18px] h-[18px]" />
    </a>
  </div>
</section>
