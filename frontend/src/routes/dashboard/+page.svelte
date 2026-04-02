<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated, user } from '$lib/stores/auth';
  import { getBookings, getVehicles } from '$lib/api';
  import {
    DollarSign, Car, Calendar, TrendingUp, Loader2, Plus
  } from 'lucide-svelte';

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
      .reduce((sum: number, b: any) => sum + (b.totalAmount || 0), 0)
  );

  let activeBookings = $derived(
    bookings.filter((b: any) => b.status === 'active' || b.status === 'confirmed').length
  );

  function formatDate(dateStr: string) {
    try {
      return new Date(dateStr).toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
    } catch {
      return dateStr;
    }
  }

  const statusColors: Record<string, string> = {
    pending: 'bg-yellow-100 text-yellow-700',
    confirmed: 'bg-blue-100 text-blue-700',
    active: 'bg-green-100 text-green-700',
    completed: 'bg-slate-100 text-slate-600',
    cancelled: 'bg-red-100 text-red-700',
  };
</script>

<svelte:head>
  <title>Dashboard - YatraSathi</title>
</svelte:head>

<div class="pt-20 pb-16 bg-slate-50 min-h-screen">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-slate-900">Dashboard</h1>
      <p class="text-slate-500 mt-1">Welcome back, {$user?.name || 'User'}</p>
    </div>

    {#if loading}
      <div class="flex items-center justify-center py-20">
        <Loader2 class="w-8 h-8 text-emerald-600 animate-spin" />
      </div>
    {:else}
      <!-- Summary Cards -->
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-6 mb-8">
        <div class="bg-white rounded-2xl p-6 shadow-sm border border-slate-100">
          <div class="flex items-center gap-4">
            <div class="w-12 h-12 bg-emerald-100 rounded-xl flex items-center justify-center">
              <DollarSign class="w-6 h-6 text-emerald-600" />
            </div>
            <div>
              <p class="text-sm text-slate-500">Total Earnings</p>
              <p class="text-2xl font-bold text-slate-900">NPR {totalEarnings.toLocaleString()}</p>
            </div>
          </div>
        </div>

        <div class="bg-white rounded-2xl p-6 shadow-sm border border-slate-100">
          <div class="flex items-center gap-4">
            <div class="w-12 h-12 bg-sky-100 rounded-xl flex items-center justify-center">
              <Calendar class="w-6 h-6 text-sky-600" />
            </div>
            <div>
              <p class="text-sm text-slate-500">Active Bookings</p>
              <p class="text-2xl font-bold text-slate-900">{activeBookings}</p>
            </div>
          </div>
        </div>

        <div class="bg-white rounded-2xl p-6 shadow-sm border border-slate-100">
          <div class="flex items-center gap-4">
            <div class="w-12 h-12 bg-amber-100 rounded-xl flex items-center justify-center">
              <Car class="w-6 h-6 text-amber-600" />
            </div>
            <div>
              <p class="text-sm text-slate-500">Vehicles Listed</p>
              <p class="text-2xl font-bold text-slate-900">{vehicles.length}</p>
            </div>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <!-- My Vehicles -->
        <div class="bg-white rounded-2xl shadow-sm border border-slate-100">
          <div class="flex items-center justify-between p-6 border-b border-slate-100">
            <h2 class="text-lg font-bold text-slate-900">My Vehicles</h2>
          </div>
          <div class="p-6">
            {#if vehicles.length === 0}
              <div class="text-center py-10">
                <Car class="w-10 h-10 text-slate-300 mx-auto mb-3" />
                <p class="text-sm text-slate-500">No vehicles listed yet</p>
              </div>
            {:else}
              <div class="space-y-3">
                {#each vehicles.slice(0, 5) as vehicle}
                  <div class="flex items-center gap-4 p-3 rounded-xl hover:bg-slate-50 transition-colors">
                    <div class="w-16 h-12 bg-slate-100 rounded-lg overflow-hidden shrink-0">
                      <img
                        src={vehicle.images?.[0] || 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=200&h=150&fit=crop'}
                        alt="{vehicle.make} {vehicle.model}"
                        class="w-full h-full object-cover"
                        onerror={(e) => { (e.target as HTMLImageElement).src = 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=200&h=150&fit=crop'; }}
                      />
                    </div>
                    <div class="flex-1 min-w-0">
                      <p class="text-sm font-semibold text-slate-900 truncate">{vehicle.make} {vehicle.model}</p>
                      <p class="text-xs text-slate-500">NPR {vehicle.pricePerDay?.toLocaleString()}/day</p>
                    </div>
                    <span class="px-2.5 py-1 text-xs font-medium rounded-lg {vehicle.isAvailable ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'}">
                      {vehicle.isAvailable ? 'Available' : 'Unavailable'}
                    </span>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>

        <!-- Recent Bookings -->
        <div class="bg-white rounded-2xl shadow-sm border border-slate-100">
          <div class="flex items-center justify-between p-6 border-b border-slate-100">
            <h2 class="text-lg font-bold text-slate-900">Recent Bookings</h2>
            <a href="/bookings" class="text-sm text-emerald-600 hover:text-emerald-700 font-medium transition-colors">
              View All
            </a>
          </div>
          <div class="p-6">
            {#if bookings.length === 0}
              <div class="text-center py-10">
                <Calendar class="w-10 h-10 text-slate-300 mx-auto mb-3" />
                <p class="text-sm text-slate-500">No bookings yet</p>
              </div>
            {:else}
              <div class="space-y-3">
                {#each bookings.slice(0, 5) as booking}
                  <div class="flex items-center justify-between p-3 rounded-xl hover:bg-slate-50 transition-colors">
                    <div class="min-w-0">
                      <p class="text-sm font-semibold text-slate-900 truncate">
                        {#if booking.vehicle}
                          {booking.vehicle.make} {booking.vehicle.model}
                        {:else}
                          Booking
                        {/if}
                      </p>
                      <p class="text-xs text-slate-500">
                        {formatDate(booking.startDate)} - {formatDate(booking.endDate)}
                      </p>
                    </div>
                    <div class="flex items-center gap-3 shrink-0">
                      <span class="text-sm font-semibold text-slate-900">
                        NPR {(booking.totalAmount || 0).toLocaleString()}
                      </span>
                      <span class="px-2 py-0.5 text-xs font-medium rounded-lg capitalize {statusColors[booking.status] || statusColors.pending}">
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
