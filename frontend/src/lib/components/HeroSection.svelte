<script lang="ts">
  import { Search, MapPin, Car, CalendarDays } from 'lucide-svelte';

  let location = $state('');
  let vehicleType = $state('');
  let date = $state('');

  function handleSearch() {
    const params = new URLSearchParams();
    if (location) params.set('location', location);
    if (vehicleType) params.set('type', vehicleType);
    const el = document.getElementById('vehicles');
    if (el) el.scrollIntoView({ behavior: 'smooth' });
  }
</script>

<section class="relative overflow-hidden bg-gradient-to-br from-emerald-800 via-emerald-700 to-emerald-900 pt-24 pb-20 md:pt-32 md:pb-28">
  <!-- Background pattern -->
  <div class="absolute inset-0 opacity-10">
    <svg class="w-full h-full" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
          <path d="M 40 0 L 0 0 0 40" fill="none" stroke="white" stroke-width="0.5"/>
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill="url(#grid)" />
    </svg>
  </div>

  <!-- Mountain silhouette -->
  <div class="absolute bottom-0 left-0 right-0">
    <svg viewBox="0 0 1440 200" class="w-full text-slate-50" fill="currentColor" preserveAspectRatio="none">
      <path d="M0,200 L0,120 L120,80 L240,100 L360,40 L480,90 L600,20 L720,70 L840,30 L960,80 L1080,50 L1200,90 L1320,60 L1440,100 L1440,200 Z" opacity="0.3"/>
      <path d="M0,200 L0,150 L180,120 L360,140 L540,100 L720,130 L900,90 L1080,120 L1260,110 L1440,140 L1440,200 Z" opacity="0.5"/>
      <path d="M0,200 L0,170 L240,155 L480,165 L720,150 L960,160 L1200,155 L1440,170 L1440,200 Z"/>
    </svg>
  </div>

  <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
    <!-- Badge -->
    <div class="inline-flex items-center gap-2 px-4 py-2 bg-white/10 backdrop-blur-sm rounded-full mb-6 border border-white/20">
      <span class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></span>
      <span class="text-sm text-emerald-100 font-medium">Nepal's #1 EV Rental Platform</span>
    </div>

    <!-- Heading -->
    <h1 class="text-4xl md:text-5xl lg:text-6xl font-extrabold text-white mb-6 leading-tight">
      Explore Nepal,
      <span class="block text-amber-400">The Electric Way</span>
    </h1>

    <p class="max-w-2xl mx-auto text-lg md:text-xl text-emerald-100 mb-10 leading-relaxed">
      Rent electric vehicles in Kathmandu. Sustainable, affordable, and ready for adventure across the valley and beyond.
    </p>

    <!-- Search Bar -->
    <div class="max-w-4xl mx-auto bg-white rounded-2xl shadow-2xl p-4 md:p-6">
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div class="relative">
          <MapPin class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-400" />
          <select
            bind:value={location}
            class="w-full pl-10 pr-4 py-3 bg-slate-50 border border-slate-200 rounded-xl text-sm text-slate-700 focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all appearance-none cursor-pointer"
          >
            <option value="">All Locations</option>
            <option value="Thamel">Thamel</option>
            <option value="Patan">Patan</option>
            <option value="Bhaktapur">Bhaktapur</option>
            <option value="Lalitpur">Lalitpur</option>
            <option value="Bouddha">Bouddha</option>
          </select>
        </div>

        <div class="relative">
          <Car class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-400" />
          <select
            bind:value={vehicleType}
            class="w-full pl-10 pr-4 py-3 bg-slate-50 border border-slate-200 rounded-xl text-sm text-slate-700 focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all appearance-none cursor-pointer"
          >
            <option value="">All Types</option>
            <option value="car">Car</option>
            <option value="suv">SUV</option>
            <option value="bike">Bike</option>
            <option value="scooter">Scooter</option>
            <option value="jeep">Jeep</option>
          </select>
        </div>

        <div class="relative">
          <CalendarDays class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-400" />
          <input
            type="date"
            bind:value={date}
            class="w-full pl-10 pr-4 py-3 bg-slate-50 border border-slate-200 rounded-xl text-sm text-slate-700 focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all"
          />
        </div>

        <button
          onclick={handleSearch}
          class="flex items-center justify-center gap-2 px-6 py-3 bg-emerald-600 text-white font-semibold rounded-xl hover:bg-emerald-700 transition-all shadow-lg shadow-emerald-600/30 hover:shadow-emerald-600/40"
        >
          <Search class="w-5 h-5" />
          Search
        </button>
      </div>
    </div>

    <!-- Quick Stats -->
    <div class="flex flex-wrap items-center justify-center gap-8 md:gap-12 mt-10 text-emerald-100">
      <div class="flex items-center gap-2">
        <span class="text-2xl font-bold text-white">500+</span>
        <span class="text-sm">EVs Available</span>
      </div>
      <div class="w-px h-8 bg-emerald-600 hidden md:block"></div>
      <div class="flex items-center gap-2">
        <span class="text-2xl font-bold text-white">10,000+</span>
        <span class="text-sm">Rides Completed</span>
      </div>
      <div class="w-px h-8 bg-emerald-600 hidden md:block"></div>
      <div class="flex items-center gap-2">
        <span class="text-2xl font-bold text-white">50 tons</span>
        <span class="text-sm">CO&#8322; Saved</span>
      </div>
    </div>
  </div>
</section>
