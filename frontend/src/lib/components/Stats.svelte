<script lang="ts">
  import { Car, Users, Leaf, MapPin } from 'lucide-svelte';

  const stats = [
    { icon: Car, value: '500+', label: 'Total Vehicles' },
    { icon: Users, value: '10,000+', label: 'Happy Riders' },
    { icon: Leaf, value: '50 tons', label: 'CO\u2082 Saved' },
    { icon: MapPin, value: '5+', label: 'Locations' },
  ];
</script>

<section class="py-12 bg-paper">
  <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
      {#each stats as stat}
        <div class="bg-white rounded-lg p-5 text-center border border-ink/5">
          <div class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-sage/10 mb-3">
            <stat.icon class="w-5 h-5 text-sage" />
          </div>
          <div class="font-display text-2xl md:text-3xl text-ink tracking-wide">{stat.value}</div>
          <div class="text-xs text-ink/40 font-mono uppercase tracking-wider mt-1">{stat.label}</div>
        </div>
      {/each}
    </div>
  </div>
</section>
