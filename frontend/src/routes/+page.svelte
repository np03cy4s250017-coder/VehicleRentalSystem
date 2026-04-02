<script lang="ts">
  import HeroSection from '$lib/components/HeroSection.svelte';
  import Stats from '$lib/components/Stats.svelte';
  import SearchFilters from '$lib/components/SearchFilters.svelte';
  import VehicleCard from '$lib/components/VehicleCard.svelte';
  import { Car } from 'lucide-svelte';

  let { data } = $props();

  let vehicleType = $state('all');
  let evOnly = $state(false);
  let sortBy = $state('');

  let filteredVehicles = $derived.by(() => {
    let result = [...(data.vehicles || [])];

    if (vehicleType !== 'all') {
      result = result.filter((v: any) => v.type === vehicleType);
    }

    if (evOnly) {
      result = result.filter((v: any) => v.is_ev);
    }

    if (sortBy === 'price-asc') {
      result.sort((a: any, b: any) => (a.daily_rate || 0) - (b.daily_rate || 0));
    } else if (sortBy === 'price-desc') {
      result.sort((a: any, b: any) => (b.daily_rate || 0) - (a.daily_rate || 0));
    } else if (sortBy === 'newest') {
      result.sort((a: any, b: any) => new Date(b.created_at || 0).getTime() - new Date(a.created_at || 0).getTime());
    }

    return result;
  });

  function onFilterChange() {
    // Reactivity handled by $derived
  }
</script>

<svelte:head>
  <title>YatraSathi - Nepal's EV Rental Marketplace</title>
</svelte:head>

<HeroSection />
<Stats />

<!-- Vehicles Section -->
<section id="vehicles" class="py-16 bg-slate-50">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <!-- Section Header -->
    <div class="text-center mb-10">
      <h2 class="text-3xl md:text-4xl font-bold text-slate-900">
        Available <span class="text-emerald-700">Vehicles</span>
      </h2>
      <p class="mt-3 text-slate-500 max-w-2xl mx-auto">
        Choose from our curated collection of electric and eco-friendly vehicles across Kathmandu Valley.
      </p>
    </div>

    <!-- Filters -->
    <SearchFilters
      bind:vehicleType
      bind:evOnly
      bind:sortBy
      onchange={onFilterChange}
    />

    <!-- Vehicle Grid -->
    {#if filteredVehicles.length > 0}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each filteredVehicles as vehicle (vehicle.id)}
          <VehicleCard {vehicle} />
        {/each}
      </div>
    {:else if data.vehicles?.length === 0}
      <!-- No vehicles from API -->
      <div class="text-center py-20">
        <div class="inline-flex items-center justify-center w-20 h-20 bg-slate-100 rounded-2xl mb-6">
          <Car class="w-10 h-10 text-slate-400" />
        </div>
        <h3 class="text-xl font-bold text-slate-900 mb-2">No Vehicles Available</h3>
        <p class="text-slate-500 max-w-md mx-auto">
          We're adding new vehicles to our fleet. Check back soon for electric rides across Kathmandu Valley!
        </p>
      </div>
    {:else}
      <!-- Filtered out all -->
      <div class="text-center py-20">
        <div class="inline-flex items-center justify-center w-20 h-20 bg-slate-100 rounded-2xl mb-6">
          <Car class="w-10 h-10 text-slate-400" />
        </div>
        <h3 class="text-xl font-bold text-slate-900 mb-2">No Vehicles Found</h3>
        <p class="text-slate-500 max-w-md mx-auto">
          Try adjusting your filters to see more results.
        </p>
        <button
          onclick={() => { vehicleType = 'all'; evOnly = false; sortBy = ''; }}
          class="mt-4 px-6 py-2 bg-emerald-600 text-white text-sm font-medium rounded-lg hover:bg-emerald-700 transition-colors"
        >
          Clear Filters
        </button>
      </div>
    {/if}
  </div>
</section>

<!-- CTA Section -->
<section class="py-20 bg-gradient-to-r from-emerald-700 to-emerald-900 text-white">
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
    <h2 class="text-3xl md:text-4xl font-bold mb-4">Own an EV? List it on YatraSathi</h2>
    <p class="text-emerald-100 text-lg mb-8 max-w-2xl mx-auto">
      Earn money by sharing your electric vehicle with riders across Kathmandu. Join Nepal's growing EV community.
    </p>
    <a
      href="/auth"
      class="inline-block px-8 py-4 bg-amber-500 text-white font-bold rounded-xl hover:bg-amber-600 transition-colors shadow-lg"
    >
      Get Started Today
    </a>
  </div>
</section>
