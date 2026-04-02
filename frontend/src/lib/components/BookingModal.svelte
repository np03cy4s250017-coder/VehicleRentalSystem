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
    <div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>

    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="relative bg-white rounded-2xl shadow-2xl w-full max-w-md max-h-[90vh] overflow-y-auto"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Header -->
      <div class="flex items-center justify-between p-6 border-b border-slate-100">
        <h2 class="text-lg font-bold text-slate-900">Book Vehicle</h2>
        <button onclick={handleClose} class="p-1 hover:bg-slate-100 rounded-lg transition-colors">
          <X class="w-5 h-5 text-slate-500" />
        </button>
      </div>

      {#if success}
        <div class="p-8 text-center">
          <div class="inline-flex items-center justify-center w-16 h-16 bg-emerald-100 rounded-full mb-4">
            <CheckCircle class="w-8 h-8 text-emerald-600" />
          </div>
          <h3 class="text-xl font-bold text-slate-900 mb-2">Booking Confirmed!</h3>
          <p class="text-sm text-slate-500 mb-6">
            Your booking for {vehicle.make} {vehicle.model} has been confirmed.
          </p>
          <a
            href="/bookings"
            class="inline-block px-6 py-3 bg-emerald-600 text-white font-semibold rounded-xl hover:bg-emerald-700 transition-colors"
          >
            View My Bookings
          </a>
        </div>
      {:else}
        <div class="p-6 space-y-5">
          <!-- Vehicle Info -->
          <div class="bg-slate-50 rounded-xl p-4">
            <p class="font-semibold text-slate-900">{vehicle.make} {vehicle.model}</p>
            <p class="text-sm text-slate-500 mt-1">
              NPR {vehicle.pricePerHour?.toLocaleString()}/hr &bull; NPR {vehicle.pricePerDay?.toLocaleString()}/day
            </p>
          </div>

          <!-- Dates -->
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label for="start-date" class="block text-sm font-medium text-slate-700 mb-1.5">Start Date</label>
              <div class="relative">
                <CalendarDays class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
                <input
                  id="start-date"
                  type="date"
                  bind:value={startDate}
                  min={new Date().toISOString().split('T')[0]}
                  class="w-full pl-10 pr-3 py-2.5 bg-slate-50 border border-slate-200 rounded-xl text-sm focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all"
                />
              </div>
            </div>
            <div>
              <label for="end-date" class="block text-sm font-medium text-slate-700 mb-1.5">End Date</label>
              <div class="relative">
                <CalendarDays class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
                <input
                  id="end-date"
                  type="date"
                  bind:value={endDate}
                  min={startDate || new Date().toISOString().split('T')[0]}
                  class="w-full pl-10 pr-3 py-2.5 bg-slate-50 border border-slate-200 rounded-xl text-sm focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all"
                />
              </div>
            </div>
          </div>

          <!-- Payment Method -->
          <div>
            <label for="payment" class="block text-sm font-medium text-slate-700 mb-1.5">Payment Method</label>
            <div class="relative">
              <CreditCard class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
              <select
                id="payment"
                bind:value={paymentMethod}
                class="w-full pl-10 pr-4 py-2.5 bg-slate-50 border border-slate-200 rounded-xl text-sm focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all appearance-none cursor-pointer"
              >
                <option value="esewa">eSewa</option>
                <option value="khalti">Khalti</option>
                <option value="cash">Cash</option>
              </select>
            </div>
          </div>

          <!-- Total -->
          {#if totalDays() > 0}
            <div class="bg-emerald-50 border border-emerald-200 rounded-xl p-4">
              <div class="flex items-center justify-between text-sm text-slate-600">
                <span>Duration</span>
                <span class="font-medium">{totalDays()} day{totalDays() > 1 ? 's' : ''}</span>
              </div>
              <div class="flex items-center justify-between mt-2">
                <span class="text-sm text-slate-600">Total Amount</span>
                <span class="text-xl font-bold text-emerald-700">NPR {totalAmount().toLocaleString()}</span>
              </div>
            </div>
          {/if}

          <!-- Error -->
          {#if error}
            <div class="p-3 bg-red-50 border border-red-200 rounded-xl text-sm text-red-600">
              {error}
            </div>
          {/if}

          <!-- Submit -->
          <button
            onclick={handleBook}
            disabled={loading}
            class="w-full py-3 bg-emerald-600 text-white font-semibold rounded-xl hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-lg shadow-emerald-600/20"
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
