<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated, user } from '$lib/stores/auth';
  import { getBookings, getVehicles } from '$lib/api';
  import { DollarSign, Car, Calendar, Loader2, ArrowRight, TrendingUp, Clock } from 'lucide-svelte';

  let vehicles = $state<any[]>([]);
  let bookings = $state<any[]>([]);
  let loading = $state(true);

  onMount(() => {
    const unsubscribe = isAuthenticated.subscribe((val) => {
      if (!val) goto('/auth');
    });
    loadData();
    return unsubscribe;
  });

  async function loadData() {
    loading = true;
    try {
      const [vehiclesRes, bookingsRes] = await Promise.all([
        getVehicles().catch(() => ({ vehicles: [] })),
        getBookings().catch(() => ({ bookings: [] })),
      ]);
      vehicles = vehiclesRes.vehicles || vehiclesRes || [];
      bookings = bookingsRes.bookings || bookingsRes || [];
    } catch {
      vehicles = [];
      bookings = [];
    } finally {
      loading = false;
    }
  }

  let totalEarnings = $derived(
    bookings
      .filter((b: any) => b.status === 'completed')
      .reduce((sum: number, b: any) => sum + (b.total_amount || 0), 0)
  );

  let activeBookings = $derived(
    bookings.filter((b: any) => b.status === 'active' || b.status === 'confirmed').length
  );

  function formatDate(dateStr: string) {
    if (!dateStr) return 'N/A';
    try {
      const d = new Date(dateStr);
      if (isNaN(d.getTime())) return dateStr.split('T')[0] || dateStr;
      return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
    } catch {
      return dateStr;
    }
  }

  const statusStyles: Record<string, string> = {
    pending: 'bg-saffron/10 text-saffron border-saffron/20',
    confirmed: 'bg-sage/10 text-sage border-sage/20',
    active: 'bg-sage/10 text-sage border-sage/20',
    completed: 'bg-ink/5 text-ink/40 border-ink/10',
    cancelled: 'bg-crimson/10 text-crimson border-crimson/20',
  };
</script>

<svelte:head>
  <title>Dashboard - YatraSathi</title>
</svelte:head>

<div class="pt-16 pb-20 bg-paper min-h-screen">
  <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">

    <!-- Welcome Header -->
    <div class="mb-8 pt-4">
      <p class="text-[13px] font-mono uppercase tracking-wider text-ink/25 mb-1">Welcome back</p>
      <h1 class="font-display text-4xl text-ink tracking-wide">{$user?.name || 'USER'}</h1>
      <p class="text-[15px] text-ink/35 mt-2">Here's an overview of your activity on YatraSathi</p>
    </div>

    {#if loading}
      <div class="flex flex-col items-center justify-center py-24">
        <Loader2 class="w-8 h-8 text-sage animate-spin mb-4" />
        <p class="text-[15px] text-ink/30">Loading dashboard...</p>
      </div>
    {:else}

      <!-- Stat Cards -->
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-5 mb-8">
        <div class="bg-white rounded-2xl p-6 border border-ink/[0.04] shadow-sm">
          <div class="flex items-center justify-between mb-4">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-sage/20 to-sage/5 flex items-center justify-center">
              <DollarSign class="w-6 h-6 text-sage" />
            </div>
            <TrendingUp class="w-5 h-5 text-sage/30" />
          </div>
          <p class="text-[13px] font-mono uppercase tracking-wider text-ink/25">Total Earnings</p>
          <p class="font-display text-3xl text-ink tracking-wide mt-1">NPR {totalEarnings.toLocaleString()}</p>
          <p class="text-[13px] text-ink/25 mt-1">From {bookings.filter(b => b.status === 'completed').length} completed bookings</p>
        </div>

        <div class="bg-white rounded-2xl p-6 border border-ink/[0.04] shadow-sm">
          <div class="flex items-center justify-between mb-4">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-saffron/20 to-saffron/5 flex items-center justify-center">
              <Calendar class="w-6 h-6 text-saffron" />
            </div>
            {#if activeBookings > 0}
              <span class="flex items-center gap-1.5 px-2.5 py-1 bg-sage/10 rounded-full">
                <span class="w-1.5 h-1.5 bg-sage rounded-full animate-pulse"></span>
                <span class="text-[12px] font-semibold text-sage">Live</span>
              </span>
            {/if}
          </div>
          <p class="text-[13px] font-mono uppercase tracking-wider text-ink/25">Active Bookings</p>
          <p class="font-display text-3xl text-ink tracking-wide mt-1">{activeBookings}</p>
          <p class="text-[13px] text-ink/25 mt-1">{bookings.length} total bookings</p>
        </div>

        <div class="bg-white rounded-2xl p-6 border border-ink/[0.04] shadow-sm">
          <div class="flex items-center justify-between mb-4">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-ink/10 to-ink/5 flex items-center justify-center">
              <Car class="w-6 h-6 text-ink/40" />
            </div>
          </div>
          <p class="text-[13px] font-mono uppercase tracking-wider text-ink/25">Vehicles Listed</p>
          <p class="font-display text-3xl text-ink tracking-wide mt-1">{vehicles.length}</p>
          <p class="text-[13px] text-ink/25 mt-1">
            {vehicles.filter(v => v.available || v.isAvailable).length} currently available
          </p>
        </div>
      </div>

      <!-- Two Column Layout -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">

        <!-- My Vehicles -->
        <div class="bg-white rounded-2xl border border-ink/[0.04] shadow-sm overflow-hidden">
          <div class="flex items-center justify-between px-6 py-4 border-b border-ink/[0.04]">
            <h2 class="text-[15px] font-semibold text-ink">My Vehicles</h2>
            <span class="text-[13px] text-ink/25">{vehicles.length} listed</span>
          </div>
          <div class="p-4">
            {#if vehicles.length === 0}
              <div class="text-center py-12">
                <div class="inline-flex items-center justify-center w-14 h-14 bg-ink/[0.03] rounded-2xl mb-4">
                  <Car class="w-7 h-7 text-ink/15" />
                </div>
                <p class="text-[15px] text-ink/30 mb-1">No vehicles listed</p>
                <p class="text-[13px] text-ink/20">Start earning by listing your vehicle</p>
              </div>
            {:else}
              <div class="space-y-2">
                {#each vehicles.slice(0, 5) as vehicle}
                  <a href="/vehicles/{vehicle.id}" class="flex items-center gap-4 p-3 rounded-xl hover:bg-paper transition-colors group">
                    <div class="w-16 h-12 bg-ink/[0.03] rounded-xl overflow-hidden shrink-0">
                      <img
                        src={vehicle.image_url || 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=200&h=150&fit=crop'}
                        alt="{vehicle.make} {vehicle.model}"
                        class="w-full h-full object-cover"
                        onerror={(e) => { (e.target as HTMLImageElement).src = 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=200&h=150&fit=crop'; }}
                      />
                    </div>
                    <div class="flex-1 min-w-0">
                      <p class="text-[15px] font-semibold text-ink truncate group-hover:text-sage transition-colors">
                        {vehicle.make} {vehicle.model}
                      </p>
                      <p class="text-[13px] text-ink/30">
                        NPR {(vehicle.daily_rate || 0).toLocaleString()}/day
                      </p>
                    </div>
                    <span class="px-2.5 py-1 text-[12px] font-semibold tracking-wider uppercase rounded-full
                      {vehicle.available || vehicle.isAvailable ? 'bg-sage/10 text-sage' : 'bg-crimson/10 text-crimson'}">
                      {vehicle.available || vehicle.isAvailable ? 'Live' : 'Off'}
                    </span>
                  </a>
                {/each}
              </div>
            {/if}
          </div>
        </div>

        <!-- Recent Bookings -->
        <div class="bg-white rounded-2xl border border-ink/[0.04] shadow-sm overflow-hidden">
          <div class="flex items-center justify-between px-6 py-4 border-b border-ink/[0.04]">
            <h2 class="text-[15px] font-semibold text-ink">Recent Bookings</h2>
            <a href="/bookings" class="flex items-center gap-1 text-[13px] text-sage font-semibold hover:underline underline-offset-2 transition-colors">
              View All <ArrowRight class="w-3.5 h-3.5" />
            </a>
          </div>
          <div class="p-4">
            {#if bookings.length === 0}
              <div class="text-center py-12">
                <div class="inline-flex items-center justify-center w-14 h-14 bg-ink/[0.03] rounded-2xl mb-4">
                  <Clock class="w-7 h-7 text-ink/15" />
                </div>
                <p class="text-[15px] text-ink/30 mb-1">No bookings yet</p>
                <p class="text-[13px] text-ink/20">Your rental history will appear here</p>
              </div>
            {:else}
              <div class="space-y-2">
                {#each bookings.slice(0, 6) as booking}
                  <div class="flex items-center justify-between p-3 rounded-xl hover:bg-paper transition-colors">
                    <div class="min-w-0 flex-1">
                      <p class="text-[15px] font-semibold text-ink truncate">
                        {#if booking.vehicle}
                          {booking.vehicle.make} {booking.vehicle.model}
                        {:else}
                          Booking
                        {/if}
                      </p>
                      <p class="text-[13px] text-ink/30 mt-0.5">
                        {formatDate(booking.start_time)} - {formatDate(booking.end_time)}
                      </p>
                    </div>
                    <div class="flex items-center gap-3 shrink-0 ml-4">
                      <span class="text-[15px] font-semibold text-ink">
                        NPR {(booking.total_amount || 0).toLocaleString()}
                      </span>
                      <span class="px-2.5 py-1 text-[12px] font-semibold tracking-wider uppercase rounded-full border
                        {statusStyles[booking.status] || statusStyles.pending}">
                        {booking.status || 'pending'}
                      </span>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>
