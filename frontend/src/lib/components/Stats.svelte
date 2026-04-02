<script lang="ts">
  import { Car, Users, Leaf, MapPin } from 'lucide-svelte';

  const stats = [
    { icon: Car, value: '500+', label: 'Total Vehicles', color: 'bg-emerald-100 text-emerald-600' },
    { icon: Users, value: '10,000+', label: 'Happy Riders', color: 'bg-amber-100 text-amber-600' },
    { icon: Leaf, value: '50 tons', label: 'CO\u2082 Saved', color: 'bg-sky-100 text-sky-600' },
    { icon: MapPin, value: '5+', label: 'Locations', color: 'bg-purple-100 text-purple-600' },
  ];
</script>

<section class="bg-slate-100 py-16">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-6">
      {#each stats as stat}
        <div class="bg-white rounded-2xl p-6 text-center shadow-sm hover:shadow-md transition-shadow">
          <div class="inline-flex items-center justify-center w-14 h-14 rounded-xl {stat.color} mb-4">
            <stat.icon class="w-7 h-7" />
          </div>
          <div class="text-2xl md:text-3xl font-bold text-slate-900 mb-1">{stat.value}</div>
          <div class="text-sm text-slate-500 font-medium">{stat.label}</div>
        </div>
      {/each}
    </div>
  </div>
</section>
