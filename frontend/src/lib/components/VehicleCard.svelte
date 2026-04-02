<script lang="ts">
  import { MapPin, Zap, Clock, IndianRupee } from 'lucide-svelte';

  interface Vehicle {
    id: string;
    make: string;
    model: string;
    year: number;
    type: string;
    is_ev: boolean;
    hourly_rate: number;
    daily_rate: number;
    location_name: string;
    features: string[];
    image_url: string;
    available: boolean;
  }

  let { vehicle }: { vehicle: Vehicle } = $props();

  const typeColors: Record<string, string> = {
    car: 'bg-blue-100 text-blue-700',
    suv: 'bg-orange-100 text-orange-700',
    bike: 'bg-purple-100 text-purple-700',
    scooter: 'bg-pink-100 text-pink-700',
    jeep: 'bg-red-100 text-red-700',
  };

  const placeholderImages: Record<string, string> = {
    car: 'https://images.unsplash.com/photo-1593941707882-a5bba14938c7?w=600&h=400&fit=crop',
    suv: 'https://images.unsplash.com/photo-1669725083850-a3e20db34781?w=600&h=400&fit=crop',
    bike: 'https://images.unsplash.com/photo-1558981285-6f0c94958bb6?w=600&h=400&fit=crop',
    scooter: 'https://images.unsplash.com/photo-1614165936528-af2006416b4b?w=600&h=400&fit=crop',
    jeep: 'https://images.unsplash.com/photo-1519641471654-76ce0107ad1b?w=600&h=400&fit=crop',
  };
</script>

<a
  href="/vehicles/{vehicle.id}"
  class="group block bg-white rounded-2xl overflow-hidden shadow-sm border border-slate-100 hover:shadow-xl hover:-translate-y-1 transition-all duration-300"
>
  <!-- Image -->
  <div class="relative aspect-[16/10] overflow-hidden bg-slate-100">
    <img
      src={vehicle.image_url || placeholderImages[vehicle.type] || placeholderImages.car}
      alt="{vehicle.make} {vehicle.model}"
      class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
      onerror={(e) => { (e.target as HTMLImageElement).src = 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=600&h=400&fit=crop'; }}
    />

    <!-- Badges -->
    <div class="absolute top-3 left-3 flex items-center gap-2">
      {#if vehicle.is_ev}
        <span class="inline-flex items-center gap-1 px-2.5 py-1 bg-sky-500 text-white text-xs font-semibold rounded-lg shadow-sm">
          <Zap class="w-3 h-3" />
          EV
        </span>
      {/if}
      <span class="px-2.5 py-1 text-xs font-semibold rounded-lg capitalize {typeColors[vehicle.type] || 'bg-slate-100 text-slate-700'}">
        {vehicle.type}
      </span>
    </div>

    {#if !vehicle.available}
      <div class="absolute inset-0 bg-black/50 flex items-center justify-center">
        <span class="px-4 py-2 bg-red-600 text-white text-sm font-bold rounded-lg">Unavailable</span>
      </div>
    {/if}
  </div>

  <!-- Content -->
  <div class="p-5">
    <!-- Title -->
    <h3 class="text-lg font-bold text-slate-900 group-hover:text-emerald-700 transition-colors">
      {vehicle.make} {vehicle.model}
    </h3>
    <p class="text-sm text-slate-500 mt-0.5">{vehicle.year}</p>

    <!-- Location -->
    <div class="flex items-center gap-1.5 mt-3 text-sm text-slate-500">
      <MapPin class="w-4 h-4 text-emerald-600" />
      {vehicle.location_name || 'Kathmandu'}
    </div>

    <!-- Features -->
    {#if vehicle.features?.length}
      <div class="flex flex-wrap gap-1.5 mt-3">
        {#each vehicle.features.slice(0, 3) as feature}
          <span class="px-2 py-0.5 bg-slate-100 text-slate-600 text-xs rounded-md font-medium">
            {feature}
          </span>
        {/each}
        {#if vehicle.features.length > 3}
          <span class="px-2 py-0.5 bg-slate-100 text-slate-400 text-xs rounded-md">
            +{vehicle.features.length - 3} more
          </span>
        {/if}
      </div>
    {/if}

    <!-- Price -->
    <div class="flex items-center justify-between mt-4 pt-4 border-t border-slate-100">
      <div class="text-sm">
        <span class="text-lg font-bold text-emerald-700">NPR {vehicle.hourly_rate?.toLocaleString()}</span>
        <span class="text-slate-400">/hr</span>
        <span class="text-slate-300 mx-1">&bull;</span>
        <span class="text-slate-600 font-medium">NPR {vehicle.daily_rate?.toLocaleString()}</span>
        <span class="text-slate-400">/day</span>
      </div>
    </div>

    <!-- Book Button -->
    <button
      class="w-full mt-4 py-2.5 bg-emerald-600 text-white text-sm font-semibold rounded-xl hover:bg-emerald-700 transition-colors shadow-sm"
    >
      Book Now
    </button>
  </div>
</a>
