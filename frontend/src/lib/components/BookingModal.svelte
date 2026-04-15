<script lang="ts">
  import { X, CalendarDays, CreditCard, CheckCircle } from 'lucide-svelte';
  import { createBooking } from '$lib/api';
  import { isAuthenticated } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  interface Props {
    vehicle: {
      _id: string;
      make: string;
      model: string;
      pricePerHour: number;
      pricePerDay: number;
    };
    show: boolean;
    onclose: () => void;
  }

  let { vehicle, show, onclose }: Props = $props();

  let startDate = $state('');
  let endDate = $state('');
  let paymentMethod = $state('esewa');
  let loading = $state(false);
  let error = $state('');
  let success = $state(false);

  let totalDays = $derived(() => {
    if (!startDate || !endDate) return 0;
    const start = new Date(startDate);
    const end = new Date(endDate);
    const diff = Math.ceil((end.getTime() - start.getTime()) / (1000 * 60 * 60 * 24));
    return diff > 0 ? diff : 0;
  });

  let totalAmount = $derived(() => {
    const days = totalDays();
    return days * (vehicle?.pricePerDay || 0);
  });

  async function handleBook() {
    if (!$isAuthenticated) {
      goto('/auth');
      return;
    }

    if (!startDate || !endDate) {
      error = 'Please select start and end dates';
      return;
    }

    if (totalDays() <= 0) {
      error = 'End date must be after start date';
      return;
    }

    loading = true;
    error = '';

    try {
      const res = await createBooking({
        vehicleId: vehicle._id,
        startDate,
        endDate,
        paymentMethod,
        totalAmount: totalAmount(),
      });

      if (res.error) {
        error = res.error;
      } else {
        success = true;
      }
    } catch {
      error = 'Failed to create booking. Please try again.';
    } finally {
      loading = false;
    }
  }

  function handleClose() {
    success = false;
    error = '';
    startDate = '';
    endDate = '';
    onclose();
  }
</script>

{#if show}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4" onclick={handleClose}>
    <div class="absolute inset-0 bg-ink/40 backdrop-blur-sm"></div>

    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="relative bg-white rounded-lg w-full max-w-md max-h-[90vh] overflow-y-auto border border-ink/5"
      onclick={(e) => e.stopPropagation()}
    >

      <div class="flex items-center justify-between p-4 border-b border-ink/5">
        <h2 class="text-sm font-semibold text-ink">Book Vehicle</h2>
        <button onclick={handleClose} class="p-1 hover:bg-paper rounded-lg transition-colors">
          <X class="w-4 h-4 text-ink/40" />
        </button>
      </div>

      {#if success}
        <div class="p-6 text-center">
          <div class="inline-flex items-center justify-center w-12 h-12 bg-sage/10 rounded-full mb-3">
            <CheckCircle class="w-6 h-6 text-sage" />
          </div>
          <h3 class="text-lg font-semibold text-ink mb-1">Booking Confirmed!</h3>
          <p class="text-xs text-ink/40 mb-4">
            Your booking for {vehicle.make} {vehicle.model} has been confirmed.
          </p>
          <a
            href="/bookings"
            class="inline-block px-5 py-2 bg-ink text-paper text-sm font-medium rounded-lg hover:bg-ink/90 transition-colors"
          >
            View My Bookings
          </a>
        </div>
      {:else}
        <div class="p-4 space-y-4">

          <div class="bg-paper rounded-lg p-3">
            <p class="text-sm font-medium text-ink">{vehicle.make} {vehicle.model}</p>
            <p class="text-xs text-ink/40 mt-0.5">
              NPR {vehicle.pricePerHour?.toLocaleString()}/hr &bull; NPR {vehicle.pricePerDay?.toLocaleString()}/day
            </p>
          </div>


          <div class="grid grid-cols-2 gap-3">
            <div>
              <label for="start-date" class="block text-xs font-medium text-ink/50 mb-1">Start Date</label>
              <div class="relative">
                <CalendarDays class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-ink/30" />
                <input
                  id="start-date"
                  type="date"
                  bind:value={startDate}
                  min={new Date().toISOString().split('T')[0]}
                  class="w-full pl-8 pr-2 py-2 bg-paper border border-ink/5 rounded-lg text-sm focus:outline-none focus:border-sage focus:ring-1 focus:ring-sage/20 transition-all"
                />
              </div>
            </div>
            <div>
              <label for="end-date" class="block text-xs font-medium text-ink/50 mb-1">End Date</label>
              <div class="relative">
                <CalendarDays class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-ink/30" />
                <input
                  id="end-date"
                  type="date"
                  bind:value={endDate}
                  min={startDate || new Date().toISOString().split('T')[0]}
                  class="w-full pl-8 pr-2 py-2 bg-paper border border-ink/5 rounded-lg text-sm focus:outline-none focus:border-sage focus:ring-1 focus:ring-sage/20 transition-all"
                />
              </div>
            </div>
          </div>


          <div>
            <label for="payment" class="block text-xs font-medium text-ink/50 mb-1">Payment Method</label>
            <div class="relative">
              <CreditCard class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-ink/30" />
              <select
                id="payment"
                bind:value={paymentMethod}
                class="w-full pl-8 pr-3 py-2 bg-paper border border-ink/5 rounded-lg text-sm focus:outline-none focus:border-sage focus:ring-1 focus:ring-sage/20 transition-all appearance-none cursor-pointer"
              >
                <option value="esewa">eSewa</option>
                <option value="khalti">Khalti</option>
                <option value="cash">Cash</option>
              </select>
            </div>
          </div>


          {#if totalDays() > 0}
            <div class="bg-sage/5 border border-sage/10 rounded-lg p-3">
              <div class="flex items-center justify-between text-xs text-ink/50">
                <span>Duration</span>
                <span class="font-medium">{totalDays()} day{totalDays() > 1 ? 's' : ''}</span>
              </div>
              <div class="flex items-center justify-between mt-1">
                <span class="text-xs text-ink/60">Total</span>
                <span class="font-display text-xl text-sage tracking-wide">NPR {totalAmount().toLocaleString()}</span>
              </div>
            </div>
          {/if}

          {#if error}
            <div class="p-2.5 bg-crimson-50 border border-crimson-200 rounded-lg text-xs text-crimson">
              {error}
            </div>
          {/if}

          <button
            onclick={handleBook}
            disabled={loading}
            class="w-full py-2.5 bg-ink text-paper text-sm font-medium rounded-lg hover:bg-ink/90 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
          >
            {#if loading}
              <span class="inline-flex items-center gap-2">
                <svg class="animate-spin w-4 h-4" viewBox="0 0 24 24" fill="none">
                  <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" class="opacity-25"/>
                  <path d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" fill="currentColor" class="opacity-75"/>
                </svg>
                Booking...
              </span>
            {:else}
              Confirm Booking
            {/if}
          </button>
        </div>
      {/if}
    </div>
  </div>
{/if}
