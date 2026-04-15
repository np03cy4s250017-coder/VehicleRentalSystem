<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated } from '$lib/stores/auth';
  import { getBookings } from '$lib/api';
  import { Calendar, CreditCard, Car, Loader2, ArrowRight, Clock } from 'lucide-svelte';

  let bookings = $state<any[]>([]);
  let loading = $state(true);
  let error = $state('');

  const statusStyles: Record<string, { bg: string; dot: string }> = {
    pending: { bg: 'bg-saffron/10 text-saffron border-saffron/20', dot: 'bg-saffron' },
    confirmed: { bg: 'bg-sage/10 text-sage border-sage/20', dot: 'bg-sage' },
    active: { bg: 'bg-sage/10 text-sage border-sage/20', dot: 'bg-sage' },
    completed: { bg: 'bg-ink/5 text-ink/40 border-ink/10', dot: 'bg-ink/30' },
    cancelled: { bg: 'bg-crimson/10 text-crimson border-crimson/20', dot: 'bg-crimson' },
  };

  const paymentLabels: Record<string, string> = {
    esewa: 'eSewa',
    khalti: 'Khalti',
    cash: 'Cash',
    connectips: 'ConnectIPS',
  };

  onMount(() => {
    const unsubscribe = isAuthenticated.subscribe((val) => {
      if (!val) goto('/auth');
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

  function formatShortDate(dateStr: string) {
    if (!dateStr) return 'N/A';
    try {
      const d = new Date(dateStr);
      if (isNaN(d.getTime())) return dateStr.split('T')[0] || dateStr;
      return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
    } catch {
      return dateStr;
    }
  }
</script>

<svelte:head>
  <title>My Bookings - YatraSathi</title>
</svelte:head>

<div class="pt-16 pb-20 bg-paper min-h-screen">
  <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">

    <div class="mb-8 pt-4">
      <h1 class="font-display text-4xl text-ink tracking-wide">MY BOOKINGS</h1>
      <p class="text-[15px] text-ink/35 mt-2">Track and manage all your vehicle rentals in one place</p>
    </div>


    {#if loading}
      <div class="flex flex-col items-center justify-center py-24">
        <Loader2 class="w-8 h-8 text-sage animate-spin mb-4" />
        <p class="text-[15px] text-ink/30">Loading your bookings...</p>
      </div>


    {:else if error}
      <div class="text-center py-24">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-crimson/5 rounded-2xl mb-4">
          <Clock class="w-8 h-8 text-crimson/40" />
        </div>
        <p class="text-[16px] text-crimson font-medium mb-4">{error}</p>
        <button
          onclick={loadBookings}
          class="px-6 py-3 bg-ink text-paper text-[15px] font-semibold rounded-xl hover:bg-ink/90 transition-colors"
        >
          Try Again
        </button>
      </div>


    {:else if bookings.length === 0}
      <div class="text-center py-24">
        <div class="inline-flex items-center justify-center w-20 h-20 bg-ink/[0.03] rounded-2xl mb-6">
          <Car class="w-10 h-10 text-ink/15" />
        </div>
        <h3 class="font-display text-2xl text-ink tracking-wide mb-2">NO BOOKINGS YET</h3>
        <p class="text-[15px] text-ink/35 max-w-md mx-auto mb-8">
          Find your perfect ride and explore the Kathmandu Valley. Your booking history will appear here.
        </p>
        <a
          href="/"
          class="inline-flex items-center gap-2 px-6 py-3 bg-ink text-paper text-[15px] font-semibold rounded-xl hover:bg-ink/90 transition-colors"
        >
          Browse Vehicles
          <ArrowRight class="w-5 h-5" />
        </a>
      </div>


    {:else}
      <div class="space-y-4">
        {#each bookings as booking}
          {@const style = statusStyles[booking.status] || statusStyles.pending}
          <div class="bg-white rounded-2xl border border-ink/[0.04] shadow-sm overflow-hidden hover:shadow-md hover:border-ink/[0.08] transition-all">
            <div class="flex flex-col sm:flex-row">

              <div class="sm:w-52 h-40 sm:h-auto bg-ink/[0.02] shrink-0 relative">
                <img
                  src={booking.vehicle?.image_url || 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=400&h=300&fit=crop'}
                  alt={booking.vehicle ? `${booking.vehicle.make} ${booking.vehicle.model}` : 'Vehicle'}
                  class="w-full h-full object-cover"
                  onerror={(e) => { (e.target as HTMLImageElement).src = 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=400&h=300&fit=crop'; }}
                />
                {#if booking.vehicle?.type}
                  <span class="absolute top-3 left-3 px-2.5 py-1 bg-white/90 backdrop-blur-sm text-ink text-[12px] font-semibold rounded-full capitalize">
                    {booking.vehicle.type}
                  </span>
                {/if}
              </div>


              <div class="flex-1 p-6">
                <div class="flex items-start justify-between gap-4">
                  <div class="min-w-0">
                    <h3 class="text-[18px] font-semibold text-ink leading-tight">
                      {#if booking.vehicle}
                        {booking.vehicle.make} {booking.vehicle.model}
                      {:else}
                        Vehicle Rental
                      {/if}
                    </h3>
                    {#if booking.vehicle?.plate_no}
                      <p class="text-[13px] text-ink/25 font-mono mt-0.5">{booking.vehicle.plate_no}</p>
                    {/if}
                  </div>


                  <span class="px-3 py-1 text-[12px] font-semibold tracking-wider uppercase rounded-full border shrink-0 {style.bg}">
                    {booking.status || 'pending'}
                  </span>
                </div>


                <div class="flex flex-wrap items-center gap-x-5 gap-y-2 mt-4">
                  <span class="flex items-center gap-2 text-[14px] text-ink/40">
                    <Calendar class="w-4 h-4 text-ink/25" />
                    {formatShortDate(booking.start_time)} - {formatShortDate(booking.end_time)}
                  </span>
                  <span class="flex items-center gap-2 text-[14px] text-ink/40">
                    <CreditCard class="w-4 h-4 text-ink/25" />
                    {paymentLabels[booking.payment_method] || booking.payment_method || 'N/A'}
                  </span>
                </div>


                <div class="mt-5 pt-4 border-t border-ink/[0.04] flex items-center justify-between">
                  <span class="text-[14px] text-ink/30">Total Amount</span>
                  <span class="font-display text-2xl text-ink tracking-wide">
                    NPR {(booking.total_amount || 0).toLocaleString()}
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
