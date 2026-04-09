<script lang="ts">
  import { SlidersHorizontal, Zap } from 'lucide-svelte';

  interface Props { vehicleType: string; evOnly: boolean; sortBy: string; onchange: () => void; }
  let { vehicleType = $bindable('all'), evOnly = $bindable(false), sortBy = $bindable(''), onchange }: Props = $props();
  function handleChange() { onchange(); }
</script>

<div class="bg-white rounded-2xl border border-ink/[0.04] p-5 mb-8">
  <div class="flex flex-col md:flex-row md:items-center gap-4">
    <div class="flex items-center gap-2.5 text-ink/40 shrink-0">
      <SlidersHorizontal class="w-[18px] h-[18px]" />
      <span class="text-[13px] font-semibold tracking-wide uppercase">Filters</span>
    </div>

    <div class="flex flex-wrap items-center gap-3 flex-1">
      <select bind:value={vehicleType} onchange={handleChange}
        class="px-4 py-2.5 bg-paper border border-ink/5 rounded-xl text-[14px] text-ink focus:outline-none focus:border-sage transition-colors cursor-pointer">
        <option value="all">All Types</option>
        <option value="car">Car</option>
        <option value="suv">SUV</option>
        <option value="bike">Bike</option>
        <option value="scooter">Scooter</option>
        <option value="jeep">Jeep</option>
      </select>

      <button onclick={() => { evOnly = !evOnly; handleChange(); }}
        class="flex items-center gap-2 px-4 py-2.5 rounded-xl text-[14px] font-medium transition-all {evOnly ? 'bg-sage text-white' : 'bg-paper text-ink/50 border border-ink/5 hover:border-ink/10'}">
        <Zap class="w-4 h-4" /> EV Only
      </button>

      <select bind:value={sortBy} onchange={handleChange}
        class="px-4 py-2.5 bg-paper border border-ink/5 rounded-xl text-[14px] text-ink focus:outline-none focus:border-sage transition-colors cursor-pointer">
        <option value="">Sort By</option>
        <option value="price-asc">Price: Low to High</option>
        <option value="price-desc">Price: High to Low</option>
        <option value="newest">Newest First</option>
      </select>
    </div>
  </div>
</div>
