<script lang="ts">
  import { MapPin, Zap } from 'lucide-svelte';

  interface Vehicle {
    id: string; make: string; model: string; year: number; type: string;
    is_ev: boolean; hourly_rate: number; daily_rate: number;
    location_name: string; features: string[]; image_url: string; available: boolean;
  }

  let { vehicle }: { vehicle: Vehicle } = $props();

  const placeholderImages: Record<string, string> = {
    car: 'https://images.unsplash.com/photo-1593941707882-a5bba14938c7?w=600&h=400&fit=crop',
    suv: 'https://images.unsplash.com/photo-1669725083850-a3e20db34781?w=600&h=400&fit=crop',
    bike: 'https://images.unsplash.com/photo-1558981285-6f0c94958bb6?w=600&h=400&fit=crop',
    scooter: 'https://images.unsplash.com/photo-1614165936528-af2006416b4b?w=600&h=400&fit=crop',
    jeep: 'https://images.unsplash.com/photo-1519641471654-76ce0107ad1b?w=600&h=400&fit=crop',
  };
</script>

<a href="/vehicles/{vehicle.id}"
  class="group block bg-white rounded-2xl overflow-hidden border border-ink/[0.04] hover:shadow-xl hover:shadow-ink/5 hover:-translate-y-0.5 transition-all duration-300">

  <div class="relative aspect-[16/10] overflow-hidden bg-paper">
    <img
      src={vehicle.image_url || placeholderImages[vehicle.type] || placeholderImages.car}
      alt="{vehicle.make} {vehicle.model}"
      class="w-full h-full object-cover group-hover:scale-[1.03] transition-transform duration-500"
      onerror={(e) => { (e.target as HTMLImageElement).src = 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=600&h=400&fit=crop'; }}
    />
    <div class="absolute top-3 left-3 flex items-center gap-2">
      {#if vehicle.is_ev}
        <span class="inline-flex items-center gap-1 px-2.5 py-1 bg-sage text-white text-[12px] font-semibold rounded-lg">
          <Zap class="w-3.5 h-3.5" /> EV
        </span>
      {/if}
      <span class="px-2.5 py-1 bg-white/90 backdrop-blur-sm text-ink text-[12px] font-semibold rounded-lg capitalize">
        {vehicle.type}
      </span>
    </div>
    {#if !vehicle.available}
      <div class="absolute inset-0 bg-ink/60 flex items-center justify-center">
        <span class="px-4 py-1.5 bg-crimson text-white text-[13px] font-semibold rounded-lg">Unavailable</span>
      </div>
    {/if}
  </div>

  <div class="p-5">
    <h3 class="text-[17px] font-semibold text-ink group-hover:text-sage transition-colors">
      {vehicle.make} {vehicle.model}
    </h3>
    <p class="text-[13px] text-ink/35 mt-0.5">{vehicle.year}</p>

    <div class="flex items-center gap-1.5 mt-2.5 text-[13px] text-ink/35">
      <MapPin class="w-3.5 h-3.5" />
      {vehicle.location_name || 'Kathmandu'}
    </div>

    {#if vehicle.features?.length}
      <div class="flex flex-wrap gap-1.5 mt-3">
        {#each vehicle.features.slice(0, 3) as feature}
          <span class="px-2 py-0.5 bg-paper text-ink/45 text-[11px] font-medium rounded-md tracking-wide uppercase">
            {feature}
          </span>
        {/each}
        {#if vehicle.features.length > 3}
          <span class="px-2 py-0.5 bg-paper text-ink/25 text-[11px] rounded-md">+{vehicle.features.length - 3}</span>
        {/if}
      </div>
    {/if}

    <div class="flex items-baseline gap-2 mt-4 pt-4 border-t border-ink/[0.04]">
      <span class="font-display text-2xl text-ink tracking-wide">NPR {vehicle.daily_rate?.toLocaleString()}</span>
      <span class="text-[13px] text-ink/30">/day</span>
      <span class="text-ink/10 mx-1">|</span>
      <span class="text-[13px] text-ink/40">NPR {vehicle.hourly_rate?.toLocaleString()}/hr</span>
    </div>
  </div>
</a>
