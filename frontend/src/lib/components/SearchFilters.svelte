<script lang="ts">
  import { SlidersHorizontal, Zap } from 'lucide-svelte';

  interface Props {
    vehicleType: string;
    evOnly: boolean;
    sortBy: string;
    onchange: () => void;
  }

  let { vehicleType = $bindable('all'), evOnly = $bindable(false), sortBy = $bindable(''), onchange }: Props = $props();

  function handleChange() {
    onchange();
  }
</script>

<div class="bg-white rounded-2xl shadow-sm border border-slate-100 p-4 md:p-6 mb-8">
  <div class="flex flex-col md:flex-row md:items-center gap-4">
    <div class="flex items-center gap-2 text-slate-700 shrink-0">
      <SlidersHorizontal class="w-5 h-5" />
      <span class="font-semibold text-sm">Filters</span>
    </div>

    <div class="flex flex-wrap items-center gap-3 flex-1">
      <!-- Vehicle Type -->
      <select
        bind:value={vehicleType}
        onchange={handleChange}
        class="px-4 py-2 bg-slate-50 border border-slate-200 rounded-lg text-sm text-slate-700 focus:outline-none focus:border-emerald-500 transition-colors cursor-pointer"
      >
        <option value="all">All Types</option>
        <option value="car">Car</option>
        <option value="suv">SUV</option>
        <option value="bike">Bike</option>
        <option value="scooter">Scooter</option>
        <option value="jeep">Jeep</option>
      </select>

      <!-- EV Only Toggle -->
      <button
        onclick={() => { evOnly = !evOnly; handleChange(); }}
        class="flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-all {evOnly ? 'bg-sky-100 text-sky-700 border border-sky-300' : 'bg-slate-50 text-slate-600 border border-slate-200 hover:border-slate-300'}"
      >
        <Zap class="w-4 h-4" />
        EV Only
      </button>

      <!-- Sort -->
      <select
        bind:value={sortBy}
        onchange={handleChange}
        class="px-4 py-2 bg-slate-50 border border-slate-200 rounded-lg text-sm text-slate-700 focus:outline-none focus:border-emerald-500 transition-colors cursor-pointer"
      >
        <option value="">Sort By</option>
        <option value="price-asc">Price: Low to High</option>
        <option value="price-desc">Price: High to Low</option>
        <option value="newest">Newest First</option>
      </select>
    </div>
  </div>
</div>
