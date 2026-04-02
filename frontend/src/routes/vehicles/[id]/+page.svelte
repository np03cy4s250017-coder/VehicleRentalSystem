<script lang="ts">
  import { goto } from '$app/navigation';
  import { isAuthenticated } from '$lib/stores/auth';
  import { createBooking } from '$lib/api';
  import {
    ArrowLeft, MapPin, Zap, Calendar, CreditCard, User, Battery, Hash,
    Tag, CheckCircle, Clock, Car, Loader2
  } from 'lucide-svelte';

  let { data } = $props();

  let vehicle = $derived(data.vehicle);

  let startDate = $state('');
  let endDate = $state('');
  let paymentMethod = $state('esewa');
  let booking = $state(false);
  let bookingError = $state('');
  let bookingSuccess = $state(false);

  let totalDays = $derived.by(() => {
    if (!startDate || !endDate) return 0;
    const start = new Date(startDate);
    const end = new Date(endDate);
    const diff = Math.ceil((end.getTime() - start.getTime()) / (1000 * 60 * 60 * 24));
    return diff > 0 ? diff : 0;
  });

  let totalAmount = $derived(totalDays * (vehicle?.daily_rate || 0));

  const typeColors: Record<string, string> = {
    car: 'bg-blue-100 text-blue-700',
    suv: 'bg-orange-100 text-orange-700',
    bike: 'bg-purple-100 text-purple-700',
    scooter: 'bg-pink-100 text-pink-700',
    jeep: 'bg-red-100 text-red-700',
  };

  const placeholderImages: Record<string, string> = {
    car: 'https://images.unsplash.com/photo-1593941707882-a5bba14938c7?w=800&h=500&fit=crop',
    suv: 'https://images.unsplash.com/photo-1669725083850-a3e20db34781?w=800&h=500&fit=crop',
    bike: 'https://images.unsplash.com/photo-1558981285-6f0c94958bb6?w=800&h=500&fit=crop',
    scooter: 'https://images.unsplash.com/photo-1614165936528-af2006416b4b?w=800&h=500&fit=crop',
    jeep: 'https://images.unsplash.com/photo-1519641471654-76ce0107ad1b?w=800&h=500&fit=crop',
  };

  async function handleBooking() {
    let authenticated = false;
    isAuthenticated.subscribe((val) => authenticated = val)();

    if (!authenticated) {
      goto('/auth');
      return;
    }

    if (!startDate || !endDate) {
      bookingError = 'Please select start and end dates';
      return;
    }

    if (totalDays <= 0) {
      bookingError = 'End date must be after start date';
      return;
    }

    booking = true;
    bookingError = '';

    try {
      const res = await createBooking({
        vehicleId: vehicle.id,
        startDate,
        endDate,
        paymentMethod,
        totalAmount,
      });

      if (res.error) {
        bookingError = res.error;
      } else {
        bookingSuccess = true;
      }
    } catch {
      bookingError = 'Failed to create booking. Please try again.';
    } finally {
      booking = false;
    }
  }
</script>

<svelte:head>
  <title>{vehicle ? `${vehicle.make} ${vehicle.model}` : 'Vehicle'} - YatraSathi</title>
</svelte:head>

<div class="pt-20 pb-16 bg-slate-50 min-h-screen">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <!-- Back Button -->
    <a
      href="/"
      class="inline-flex items-center gap-2 text-sm text-slate-600 hover:text-emerald-700 transition-colors mb-6"
    >
      <ArrowLeft class="w-4 h-4" />
      Back to vehicles
    </a>

    {#if vehicle}
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <!-- Left: Vehicle Details -->
        <div class="lg:col-span-2 space-y-6">
          <!-- Hero Image -->
          <div class="relative rounded-2xl overflow-hidden bg-slate-200 aspect-[16/9]">
            <img
              src={vehicle.image_url || placeholderImages[vehicle.type] || placeholderImages.car}
              alt="{vehicle.make} {vehicle.model}"
              class="w-full h-full object-cover"
              onerror={(e) => { (e.target as HTMLImageElement).src = 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=800&h=500&fit=crop'; }}
            />
            <div class="absolute top-4 left-4 flex items-center gap-2">
              {#if vehicle.is_ev}
                <span class="inline-flex items-center gap-1 px-3 py-1.5 bg-sky-500 text-white text-sm font-semibold rounded-lg shadow-sm">
                  <Zap class="w-4 h-4" />
                  Electric Vehicle
                </span>
              {/if}
              <span class="px-3 py-1.5 text-sm font-semibold rounded-lg capitalize {typeColors[vehicle.type] || 'bg-slate-100 text-slate-700'}">
                {vehicle.type}
              </span>
            </div>
          </div>

          <!-- Vehicle Info -->
          <div class="bg-white rounded-2xl p-6 shadow-sm border border-slate-100">
            <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-4">
              <div>
                <h1 class="text-2xl md:text-3xl font-bold text-slate-900">
                  {vehicle.make} {vehicle.model}
                </h1>
                <p class="text-slate-500 mt-1">{vehicle.year}</p>
              </div>
              <div class="flex items-center gap-2 text-slate-500">
                <MapPin class="w-5 h-5 text-emerald-600" />
                <span class="font-medium">{vehicle.location_name || 'Kathmandu'}</span>
              </div>
            </div>

            <!-- Features -->
            {#if vehicle.features?.length}
              <div class="mt-6">
                <h3 class="text-sm font-semibold text-slate-700 mb-3">Features</h3>
                <div class="flex flex-wrap gap-2">
                  {#each vehicle.features as feature}
                    <span class="px-3 py-1.5 bg-slate-100 text-slate-700 text-sm rounded-lg font-medium">
                      {feature}
                    </span>
                  {/each}
                </div>
              </div>
            {/if}

            <!-- Specs -->
            <div class="mt-6 grid grid-cols-2 sm:grid-cols-3 gap-4">
              <div class="bg-slate-50 rounded-xl p-4">
                <Car class="w-5 h-5 text-slate-400 mb-2" />
                <p class="text-xs text-slate-500">Type</p>
                <p class="text-sm font-semibold text-slate-900 capitalize">{vehicle.type}</p>
              </div>
              {#if vehicle.is_ev && vehicle.ev_range_km}
                <div class="bg-slate-50 rounded-xl p-4">
                  <Battery class="w-5 h-5 text-green-500 mb-2" />
                  <p class="text-xs text-slate-500">EV Range</p>
                  <p class="text-sm font-semibold text-slate-900">{vehicle.ev_range_km} km</p>
                </div>
              {/if}
              {#if vehicle.plate_no}
                <div class="bg-slate-50 rounded-xl p-4">
                  <Hash class="w-5 h-5 text-slate-400 mb-2" />
                  <p class="text-xs text-slate-500">Plate</p>
                  <p class="text-sm font-semibold text-slate-900">{vehicle.plate_no}</p>
                </div>
              {/if}
              {#if vehicle.listing_type}
                <div class="bg-slate-50 rounded-xl p-4">
                  <Tag class="w-5 h-5 text-slate-400 mb-2" />
                  <p class="text-xs text-slate-500">Listing</p>
                  <p class="text-sm font-semibold text-slate-900 capitalize">{vehicle.listing_type}</p>
                </div>
              {/if}
            </div>
          </div>

          <!-- Owner Info -->
          {#if vehicle.owner}
            <div class="bg-white rounded-2xl p-6 shadow-sm border border-slate-100">
              <h3 class="text-sm font-semibold text-slate-700 mb-4">Listed By</h3>
              <div class="flex items-center gap-4">
                <div class="w-12 h-12 rounded-full bg-emerald-100 flex items-center justify-center">
                  <User class="w-6 h-6 text-emerald-600" />
                </div>
                <div>
                  <p class="font-semibold text-slate-900">{vehicle.owner.name || 'Vehicle Owner'}</p>
                  <p class="text-sm text-slate-500">Verified Owner</p>
                </div>
              </div>
            </div>
          {/if}

          <!-- Map Placeholder -->
          <div class="bg-white rounded-2xl p-6 shadow-sm border border-slate-100">
            <h3 class="text-sm font-semibold text-slate-700 mb-4">Location</h3>
            <div class="bg-emerald-50 rounded-xl h-48 flex items-center justify-center border border-emerald-100">
              <div class="text-center">
                <MapPin class="w-8 h-8 text-emerald-500 mx-auto mb-2" />
                <p class="text-sm text-emerald-700 font-medium">{vehicle.location_name || 'Kathmandu'}, Nepal</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Right: Booking Card -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-2xl p-6 shadow-sm border border-slate-100 sticky top-24">
            {#if bookingSuccess}
              <div class="text-center py-6">
                <div class="inline-flex items-center justify-center w-16 h-16 bg-emerald-100 rounded-full mb-4">
                  <CheckCircle class="w-8 h-8 text-emerald-600" />
                </div>
                <h3 class="text-xl font-bold text-slate-900 mb-2">Booking Confirmed!</h3>
                <p class="text-sm text-slate-500 mb-6">Your ride is secured.</p>
                <a
                  href="/bookings"
                  class="inline-block px-6 py-3 bg-emerald-600 text-white font-semibold rounded-xl hover:bg-emerald-700 transition-colors"
                >
                  View Bookings
                </a>
              </div>
            {:else}
              <!-- Pricing -->
              <div class="mb-6">
                <h3 class="text-lg font-bold text-slate-900 mb-3">Pricing</h3>
                <div class="space-y-2">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2 text-sm text-slate-600">
                      <Clock class="w-4 h-4" />
                      Per Hour
                    </div>
                    <span class="text-lg font-bold text-emerald-700">NPR {vehicle.hourly_rate?.toLocaleString()}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2 text-sm text-slate-600">
                      <Calendar class="w-4 h-4" />
                      Per Day
                    </div>
                    <span class="text-lg font-bold text-emerald-700">NPR {vehicle.daily_rate?.toLocaleString()}</span>
                  </div>
                </div>
              </div>

              <hr class="border-slate-100 mb-6" />

              <!-- Booking Form -->
              <div class="space-y-4">
                <div>
                  <label for="detail-start" class="block text-sm font-medium text-slate-700 mb-1.5">Start Date</label>
                  <input
                    id="detail-start"
                    type="date"
                    bind:value={startDate}
                    min={new Date().toISOString().split('T')[0]}
                    class="w-full px-4 py-2.5 bg-slate-50 border border-slate-200 rounded-xl text-sm focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all"
                  />
                </div>
                <div>
                  <label for="detail-end" class="block text-sm font-medium text-slate-700 mb-1.5">End Date</label>
                  <input
                    id="detail-end"
                    type="date"
                    bind:value={endDate}
                    min={startDate || new Date().toISOString().split('T')[0]}
                    class="w-full px-4 py-2.5 bg-slate-50 border border-slate-200 rounded-xl text-sm focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all"
                  />
                </div>
                <div>
                  <label for="detail-payment" class="block text-sm font-medium text-slate-700 mb-1.5">Payment Method</label>
                  <select
                    id="detail-payment"
                    bind:value={paymentMethod}
                    class="w-full px-4 py-2.5 bg-slate-50 border border-slate-200 rounded-xl text-sm focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all appearance-none cursor-pointer"
                  >
                    <option value="esewa">eSewa</option>
                    <option value="khalti">Khalti</option>
                    <option value="cash">Cash on Delivery</option>
                  </select>
                </div>

                {#if totalDays > 0}
                  <div class="bg-emerald-50 border border-emerald-200 rounded-xl p-4">
                    <div class="flex items-center justify-between text-sm text-slate-600 mb-1">
                      <span>Duration</span>
                      <span class="font-medium">{totalDays} day{totalDays > 1 ? 's' : ''}</span>
                    </div>
                    <div class="flex items-center justify-between">
                      <span class="text-sm font-medium text-slate-700">Total</span>
                      <span class="text-xl font-bold text-emerald-700">NPR {totalAmount.toLocaleString()}</span>
                    </div>
                  </div>
                {/if}

                {#if bookingError}
                  <div class="p-3 bg-red-50 border border-red-200 rounded-xl text-sm text-red-600">
                    {bookingError}
                  </div>
                {/if}

                <button
                  onclick={handleBooking}
                  disabled={booking || !vehicle.available}
                  class="w-full py-3 bg-emerald-600 text-white font-semibold rounded-xl hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-lg shadow-emerald-600/20"
                >
                  {#if booking}
                    <span class="inline-flex items-center gap-2">
                      <Loader2 class="w-4 h-4 animate-spin" />
                      Booking...
                    </span>
                  {:else if !vehicle.available}
                    Currently Unavailable
                  {:else}
                    Book Now
                  {/if}
                </button>
              </div>
            {/if}
          </div>
        </div>
      </div>
    {:else}
      <!-- Vehicle Not Found -->
      <div class="text-center py-20">
        <div class="inline-flex items-center justify-center w-20 h-20 bg-slate-100 rounded-2xl mb-6">
          <Car class="w-10 h-10 text-slate-400" />
        </div>
        <h2 class="text-2xl font-bold text-slate-900 mb-2">Vehicle Not Found</h2>
        <p class="text-slate-500 mb-6">This vehicle may have been removed or doesn't exist.</p>
        <a
          href="/"
          class="inline-block px-6 py-3 bg-emerald-600 text-white font-semibold rounded-xl hover:bg-emerald-700 transition-colors"
        >
          Browse Vehicles
        </a>
      </div>
    {/if}
  </div>
</div>
