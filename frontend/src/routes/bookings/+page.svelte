<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated } from '$lib/stores/auth';
  import { getBookings } from '$lib/api';
  import { Calendar, CreditCard, Car, Clock, Loader2, ArrowRight } from 'lucide-svelte';

  let bookings = $state<any[]>([]);
  let loading = $state(true);
  let error = $state('');

  const statusColors: Record<string, string> = {
    pending: 'bg-yellow-100 text-yellow-700 border-yellow-200',
    confirmed: 'bg-blue-100 text-blue-700 border-blue-200',
    active: 'bg-green-100 text-green-700 border-green-200',
    completed: 'bg-slate-100 text-slate-600 border-slate-200',
    cancelled: 'bg-red-100 text-red-700 border-red-200',
  };

  onMount(() => {
    const unsubscribe = isAuthenticated.subscribe((val) => {
      if (!val) {
        goto('/auth');
      }
    });

    loadBookings();
    return unsubscribe;
  });

  async function loadBookings() {
    loading = true;
    try {
      const res = await getBookings();
      bookings = res.bookings || res || [];
    } catch {
      error = 'Failed to load bookings';
      bookings = [];
    } finally {
      loading = false;
    }
  }

  function formatDate(dateStr: string) {
    try {
      return new Date(dateStr).toLocaleDateString('en-US', {
        year: 'numeric', month: 'short', day: 'numeric'
      });
    } catch {
      return dateStr;
    }
  }
</script>

<svelte:head>
  <title>My Bookings - YatraSathi</title>
</svelte:head>

<div class="pt-20 pb-16 bg-slate-50 min-h-screen">
  <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-slate-900">My Bookings</h1>
      <p class="text-slate-500 mt-1">Track and manage your vehicle rentals</p>
    </div>

    {#if loading}
      <div class="flex items-center justify-center py-20">
        <Loader2 class="w-8 h-8 text-emerald-600 animate-spin" />
      </div>
    {:else if error}
      <div class="text-center py-20">
        <p class="text-red-600 mb-4">{error}</p>
        <button
          onclick={loadBookings}
          class="px-6 py-2 bg-emerald-600 text-white text-sm font-medium rounded-lg hover:bg-emerald-700 transition-colors"
        >
          Retry
        </button>
      </div>
    {:else if bookings.length === 0}
      <!-- Empty State -->
      <div class="text-center py-20">
        <div class="inline-flex items-center justify-center w-20 h-20 bg-slate-100 rounded-2xl mb-6">
          <Car class="w-10 h-10 text-slate-400" />
        </div>
        <h3 class="text-xl font-bold text-slate-900 mb-2">No Bookings Yet</h3>
        <p class="text-slate-500 max-w-md mx-auto mb-6">
          Find your perfect ride and explore Kathmandu Valley the electric way!
        </p>
        <a
          href="/"
          class="inline-flex items-center gap-2 px-6 py-3 bg-emerald-600 text-white font-semibold rounded-xl hover:bg-emerald-700 transition-colors"
        >
          Browse Vehicles
          <ArrowRight class="w-5 h-5" />
        </a>
      </div>
    {:else}
      <div class="space-y-4">
        {#each bookings as booking}
          <div class="bg-white rounded-2xl shadow-sm border border-slate-100 overflow-hidden hover:shadow-md transition-shadow">
            <div class="flex flex-col sm:flex-row">
              <!-- Vehicle Image -->
              <div class="sm:w-48 h-40 sm:h-auto bg-slate-100 shrink-0">
                <img
                  src={booking.vehicle?.images?.[0] || 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=400&h=300&fit=crop'}
                  alt={booking.vehicle ? `${booking.vehicle.make} ${booking.vehicle.model}` : 'Vehicle'}
                  class="w-full h-full object-cover"
                  onerror={(e) => { (e.target as HTMLImageElement).src = 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=400&h=300&fit=crop'; }}
                />
              </div>

              <!-- Booking Details -->
              <div class="flex-1 p-5">
                <div class="flex items-start justify-between gap-4">
                  <div>
                    <h3 class="text-lg font-bold text-slate-900">
                      {#if booking.vehicle}
                        {booking.vehicle.make} {booking.vehicle.model}
                      {:else}
                        Vehicle Rental
                      {/if}
                    </h3>
                    <div class="flex flex-wrap items-center gap-4 mt-2 text-sm text-slate-500">
                      <span class="flex items-center gap-1.5">
                        <Calendar class="w-4 h-4" />
                        {formatDate(booking.startDate)} - {formatDate(booking.endDate)}
                      </span>
                      <span class="flex items-center gap-1.5">
                        <CreditCard class="w-4 h-4" />
                        {booking.paymentMethod || 'N/A'}
                      </span>
                    </div>
                  </div>

                  <!-- Status -->
                  <span class="px-3 py-1 text-xs font-semibold rounded-lg capitalize border {statusColors[booking.status] || statusColors.pending}">
                    {booking.status || 'pending'}
                  </span>
                </div>

                <!-- Amount -->
                <div class="mt-4 pt-3 border-t border-slate-100 flex items-center justify-between">
                  <span class="text-sm text-slate-500">Total Amount</span>
                  <span class="text-lg font-bold text-emerald-700">
                    NPR {(booking.totalAmount || 0).toLocaleString()}
                  </span>
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
