<script lang="ts">
  import { Search, MapPin, Car, CalendarDays } from 'lucide-svelte';

  interface Props {
    onSearch?: (filters: { type: string; location: string }) => void;
  }

  let { onSearch }: Props = $props();

  let location = $state('');
  let vehicleType = $state('');
  let date = $state('');

  function handleSearch() {
    if (onSearch) {
      onSearch({ type: vehicleType, location });
    }
    const el = document.getElementById('vehicles');
    if (el) el.scrollIntoView({ behavior: 'smooth' });
  }
</script>

<section class="relative pt-32 pb-28 md:pt-40 md:pb-36 bg-ink overflow-hidden">
  <div class="absolute bottom-0 left-0 right-0 opacity-[0.04]">
    <svg viewBox="0 0 1440 200" class="w-full text-paper" fill="currentColor" preserveAspectRatio="none">
      <path d="M0,200 L0,120 L120,80 L240,100 L360,40 L480,90 L600,20 L720,70 L840,30 L960,80 L1080,50 L1200,90 L1320,60 L1440,100 L1440,200 Z"/>
    </svg>
  </div>

  <div class="relative max-w-7xl mx-auto px-6 lg:px-8">
    <div class="flex justify-center mb-10">
      <div class="inline-flex items-center gap-2.5 px-4 py-1.5 bg-white/5 rounded-full border border-white/10">
        <span class="w-2 h-2 bg-sage rounded-full"></span>
        <span class="text-[13px] text-paper/50 font-medium tracking-wide">Kathmandu's EV Mobility Platform</span>
      </div>
    </div>

    <h1 class="text-center">
      <span class="block font-display text-6xl md:text-8xl lg:text-9xl text-paper tracking-wide leading-[0.9]">
        EXPLORE NEPAL
      </span>
      <span class="block font-display text-6xl md:text-8xl lg:text-9xl text-saffron tracking-wide leading-[0.9] mt-2">
        THE ELECTRIC WAY
      </span>
    </h1>

    <p class="max-w-2xl mx-auto text-center text-paper/40 mt-8 text-lg md:text-xl leading-relaxed">
      Rent electric vehicles across Kathmandu Valley. Sustainable, affordable, and ready for your next adventure.
    </p>


    <div class="max-w-4xl mx-auto mt-12 bg-white rounded-2xl p-4 shadow-2xl shadow-black/10">
      <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
        <div class="relative">
          <MapPin class="absolute left-3.5 top-1/2 -translate-y-1/2 w-[18px] h-[18px] text-ink/25" />
          <select bind:value={location}
            class="w-full pl-10 pr-4 py-3 bg-paper/60 border border-ink/5 rounded-xl text-[14px] text-ink focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all appearance-none cursor-pointer">
            <option value="">All Locations</option>
            <option value="Thamel">Thamel</option>
            <option value="Patan">Patan</option>
            <option value="Bhaktapur">Bhaktapur</option>
            <option value="Lalitpur">Lalitpur</option>
            <option value="Bouddha">Bouddha</option>
          </select>
        </div>
        <div class="relative">
          <Car class="absolute left-3.5 top-1/2 -translate-y-1/2 w-[18px] h-[18px] text-ink/25" />
          <select bind:value={vehicleType}
            class="w-full pl-10 pr-4 py-3 bg-paper/60 border border-ink/5 rounded-xl text-[14px] text-ink focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all appearance-none cursor-pointer">
            <option value="">All Types</option>
            <option value="car">Car</option>
            <option value="suv">SUV</option>
            <option value="bike">Bike</option>
            <option value="scooter">Scooter</option>
            <option value="jeep">Jeep</option>
          </select>
        </div>
        <div class="relative">
          <CalendarDays class="absolute left-3.5 top-1/2 -translate-y-1/2 w-[18px] h-[18px] text-ink/25" />
          <input type="date" bind:value={date}
            class="w-full pl-10 pr-4 py-3 bg-paper/60 border border-ink/5 rounded-xl text-[14px] text-ink focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all" />
        </div>
        <button onclick={handleSearch}
          class="flex items-center justify-center gap-2 px-6 py-3 bg-sage text-white text-[15px] font-semibold rounded-xl hover:bg-sage-600 transition-all">
          <Search class="w-[18px] h-[18px]" />
          Search
        </button>
      </div>
    </div>


    <div class="flex flex-wrap items-center justify-center gap-12 md:gap-20 mt-16">
      <div class="text-center">
        <span class="block font-display text-4xl md:text-5xl text-paper tracking-wide">500+</span>
        <span class="text-[13px] text-paper/30 font-medium tracking-wider uppercase mt-1">Electric Vehicles</span>
      </div>
      <div class="w-px h-12 bg-paper/10 hidden md:block"></div>
      <div class="text-center">
        <span class="block font-display text-4xl md:text-5xl text-paper tracking-wide">10K+</span>
        <span class="text-[13px] text-paper/30 font-medium tracking-wider uppercase mt-1">Rides Completed</span>
      </div>
      <div class="w-px h-12 bg-paper/10 hidden md:block"></div>
      <div class="text-center">
        <span class="block font-display text-4xl md:text-5xl text-paper tracking-wide">50T</span>
        <span class="text-[13px] text-paper/30 font-medium tracking-wider uppercase mt-1">CO&#8322; Saved</span>
      </div>
    </div>
  </div>
</section>
