<script lang="ts">
  import { goto } from '$app/navigation';
  import { isAuthenticated } from '$lib/stores/auth';
  import { createBooking, verifyEsewaPayment, verifyKhaltiPayment } from '$lib/api';
  import {
    ArrowLeft, MapPin, Zap, Calendar, User, Battery, Hash,
    Tag, CheckCircle, Clock, Car, Loader2, CreditCard, Banknote, Wallet
  } from 'lucide-svelte';

  let { data } = $props();
  let vehicle = $derived(data.vehicle);

  let startDate = $state('');
  let endDate = $state('');
  let paymentMethod = $state('esewa');
  let booking = $state(false);
  let bookingError = $state('');
  let bookingSuccess = $state(false);
  let bookingId = $state('');
  let bookingAmount = $state(0);
  let paymentStep = $state(false);
  let paymentVerifying = $state(false);
  let paymentSuccess = $state(false);

  let totalDays = $derived.by(() => {
    if (!startDate || !endDate) return 0;
    const start = new Date(startDate);
    const end = new Date(endDate);
    const diff = Math.ceil((end.getTime() - start.getTime()) / (1000 * 60 * 60 * 24));
    return diff > 0 ? diff : 0;
  });

  let totalAmount = $derived(totalDays * (vehicle?.daily_rate || 0));

  const placeholderImages: Record<string, string> = {
    car: 'https://images.unsplash.com/photo-1593941707882-a5bba14938c7?w=800&h=500&fit=crop',
    suv: 'https://images.unsplash.com/photo-1669725083850-a3e20db34781?w=800&h=500&fit=crop',
    bike: 'https://images.unsplash.com/photo-1558981285-6f0c94958bb6?w=800&h=500&fit=crop',
    scooter: 'https://images.unsplash.com/photo-1614165936528-af2006416b4b?w=800&h=500&fit=crop',
    jeep: 'https://images.unsplash.com/photo-1519641471654-76ce0107ad1b?w=800&h=500&fit=crop',
  };

  function getImage(v: any) {
    return v?.image_url || placeholderImages[v?.type] || placeholderImages.car;
  }

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
        bookingId = res.booking?.id || '';
        bookingAmount = res.booking?.total_amount || totalAmount;

        if (paymentMethod === 'cash') {
          bookingSuccess = true;
        } else {
          paymentStep = true;
        }
      }
    } catch {
      bookingError = 'Failed to create booking. Please try again.';
    } finally {
      booking = false;
    }
  }

  async function handlePaymentVerify() {
    if (!bookingId) return;

    paymentVerifying = true;
    bookingError = '';

    try {
      let res;
      if (paymentMethod === 'esewa') {
        res = await verifyEsewaPayment({
          bookingId,
          referenceId: `esewa-ref-${Date.now()}`,
          productId: `ys-booking-${bookingId}`,
          amount: bookingAmount,
        });
      } else if (paymentMethod === 'khalti') {
        res = await verifyKhaltiPayment({
          bookingId,
          token: `khalti-token-${Date.now()}`,
          amount: bookingAmount,
        });
      }

      if (res?.success) {
        paymentSuccess = true;
        bookingSuccess = true;
        paymentStep = false;
      } else {
        bookingError = res?.error || 'Payment verification failed';
      }
    } catch {
      bookingError = 'Payment verification failed. Please try again.';
    } finally {
      paymentVerifying = false;
    }
  }
</script>

<svelte:head>
  <title>{vehicle ? `${vehicle.make} ${vehicle.model}` : 'Vehicle'} - YatraSathi</title>
</svelte:head>

<div class="pt-16 pb-20 bg-paper min-h-screen">
  <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
    <!-- Back navigation -->
    <a
      href="/"
      class="inline-flex items-center gap-2 text-[15px] text-ink/35 hover:text-ink transition-colors mb-8 group"
    >
      <ArrowLeft class="w-4 h-4 group-hover:-translate-x-0.5 transition-transform" />
      Back to vehicles
    </a>

    {#if vehicle}
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <!-- Left Column: Vehicle Details -->
        <div class="lg:col-span-2 space-y-6">

          <!-- Hero Image -->
          <div class="relative rounded-2xl overflow-hidden bg-ink/5 aspect-[16/9] shadow-sm">
            <img
              src={getImage(vehicle)}
              alt="{vehicle.make} {vehicle.model}"
              class="w-full h-full object-cover"
              onerror={(e) => { (e.target as HTMLImageElement).src = 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=800&h=500&fit=crop'; }}
            />
            <div class="absolute top-4 left-4 flex items-center gap-2">
              {#if vehicle.is_ev}
                <span class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-sage text-white text-[13px] font-semibold rounded-full shadow-lg">
                  <Zap class="w-3.5 h-3.5" />
                  Electric
                </span>
              {/if}
              <span class="px-3 py-1.5 bg-white/90 backdrop-blur-sm text-ink text-[13px] font-semibold rounded-full capitalize shadow-lg">
                {vehicle.type}
              </span>
            </div>
            {#if !vehicle.available}
              <div class="absolute inset-0 bg-ink/40 flex items-center justify-center">
                <span class="px-5 py-2 bg-white text-ink text-[15px] font-semibold rounded-full">Currently Unavailable</span>
              </div>
            {/if}
          </div>

          <!-- Vehicle Info Card -->
          <div class="bg-white rounded-2xl p-8 border border-ink/[0.04] shadow-sm">
            <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-4">
              <div>
                <h1 class="font-display text-4xl text-ink tracking-wide">
                  {vehicle.make} {vehicle.model}
                </h1>
                <p class="text-[15px] text-ink/35 mt-1">{vehicle.year} Model</p>
              </div>
              <div class="flex items-center gap-2 text-ink/40 bg-paper px-4 py-2 rounded-xl">
                <MapPin class="w-4 h-4 text-sage" />
                <span class="text-[15px] font-medium">{vehicle.location_name || 'Kathmandu'}</span>
              </div>
            </div>

            {#if vehicle.features?.length}
              <div class="mt-8">
                <h3 class="text-[13px] font-mono uppercase tracking-wider text-ink/30 mb-3">Features</h3>
                <div class="flex flex-wrap gap-2">
                  {#each vehicle.features as feature}
                    <span class="px-3 py-1.5 bg-paper text-ink/60 text-[14px] rounded-lg font-medium border border-ink/[0.04]">{feature}</span>
                  {/each}
                </div>
              </div>
            {/if}

            <!-- Specs Grid -->
            <div class="mt-8 grid grid-cols-2 sm:grid-cols-4 gap-4">
              <div class="bg-paper rounded-xl p-4">
                <Car class="w-5 h-5 text-ink/25 mb-2" />
                <p class="text-[12px] font-mono uppercase tracking-wider text-ink/25">Type</p>
                <p class="text-[15px] font-semibold text-ink capitalize mt-0.5">{vehicle.type}</p>
              </div>
              {#if vehicle.is_ev && vehicle.ev_range_km}
                <div class="bg-paper rounded-xl p-4">
                  <Battery class="w-5 h-5 text-sage mb-2" />
                  <p class="text-[12px] font-mono uppercase tracking-wider text-ink/25">EV Range</p>
                  <p class="text-[15px] font-semibold text-ink mt-0.5">{vehicle.ev_range_km} km</p>
                </div>
              {/if}
              {#if vehicle.plate_no}
                <div class="bg-paper rounded-xl p-4">
                  <Hash class="w-5 h-5 text-ink/25 mb-2" />
                  <p class="text-[12px] font-mono uppercase tracking-wider text-ink/25">Plate</p>
                  <p class="text-[15px] font-semibold text-ink mt-0.5">{vehicle.plate_no}</p>
                </div>
              {/if}
              {#if vehicle.listing_type}
                <div class="bg-paper rounded-xl p-4">
                  <Tag class="w-5 h-5 text-ink/25 mb-2" />
                  <p class="text-[12px] font-mono uppercase tracking-wider text-ink/25">Listing</p>
                  <p class="text-[15px] font-semibold text-ink capitalize mt-0.5">{vehicle.listing_type}</p>
                </div>
              {/if}
            </div>
          </div>

          <!-- Owner Card -->
          {#if vehicle.owner}
            <div class="bg-white rounded-2xl p-6 border border-ink/[0.04] shadow-sm">
              <h3 class="text-[13px] font-mono uppercase tracking-wider text-ink/30 mb-4">Listed By</h3>
              <div class="flex items-center gap-4">
                <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-sage/20 to-sage/5 flex items-center justify-center">
                  <User class="w-6 h-6 text-sage" />
                </div>
                <div>
                  <p class="text-[16px] font-semibold text-ink">{vehicle.owner.name || 'Vehicle Owner'}</p>
                  <p class="text-[14px] text-ink/35">Verified Owner</p>
                </div>
              </div>
            </div>
          {/if}

          <!-- Location Card -->
          <div class="bg-white rounded-2xl p-6 border border-ink/[0.04] shadow-sm">
            <h3 class="text-[13px] font-mono uppercase tracking-wider text-ink/30 mb-4">Location</h3>
            <div class="bg-sage/[0.04] rounded-xl h-48 flex items-center justify-center border border-sage/10">
              <div class="text-center">
                <MapPin class="w-8 h-8 text-sage/60 mx-auto mb-2" />
                <p class="text-[16px] text-sage font-semibold">{vehicle.location_name || 'Kathmandu'}, Nepal</p>
                <p class="text-[13px] text-ink/25 mt-1">Map integration coming soon</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Right Sidebar: Booking -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-2xl p-6 border border-ink/[0.04] shadow-sm sticky top-24">

            <!-- Success State -->
            {#if bookingSuccess}
              <div class="text-center py-8">
                <div class="inline-flex items-center justify-center w-16 h-16 bg-sage/10 rounded-2xl mb-4">
                  <CheckCircle class="w-8 h-8 text-sage" />
                </div>
                <h3 class="font-display text-2xl text-ink tracking-wide mb-2">
                  {paymentSuccess ? 'PAYMENT CONFIRMED' : 'BOOKING CREATED'}
                </h3>
                <p class="text-[14px] text-ink/40 mb-2">
                  {paymentSuccess
                    ? `NPR ${bookingAmount.toLocaleString()} paid via ${paymentMethod === 'esewa' ? 'eSewa' : 'Khalti'}`
                    : paymentMethod === 'cash' ? 'Pay at pickup. Your ride is secured.' : 'Your ride is secured.'}
                </p>
                {#if bookingId}
                  <div class="inline-block px-3 py-1.5 bg-paper rounded-lg mb-6">
                    <p class="text-[13px] text-ink/30 font-mono">ID: {bookingId.slice(0, 8)}...</p>
                  </div>
                {/if}
                <a
                  href="/bookings"
                  class="inline-flex items-center gap-2 px-6 py-3 bg-ink text-paper text-[15px] font-semibold rounded-xl hover:bg-ink/90 transition-colors"
                >
                  View My Bookings
                </a>
              </div>

            <!-- Payment Step -->
            {:else if paymentStep}
              <div class="space-y-5">
                <div class="text-center">
                  <div class="inline-flex items-center justify-center w-14 h-14 rounded-2xl mb-4
                    {paymentMethod === 'esewa' ? 'bg-green-100' : 'bg-purple-100'}">
                    <CreditCard class="w-6 h-6 {paymentMethod === 'esewa' ? 'text-green-600' : 'text-purple-600'}" />
                  </div>
                  <h3 class="font-display text-xl text-ink tracking-wide">
                    PAY WITH {paymentMethod === 'esewa' ? 'ESEWA' : 'KHALTI'}
                  </h3>
                  <p class="text-[14px] text-ink/40 mt-1">Complete payment to confirm booking</p>
                </div>

                <div class="bg-paper rounded-xl p-5 text-center">
                  <p class="text-[13px] font-mono uppercase tracking-wider text-ink/25">Amount Due</p>
                  <p class="font-display text-4xl text-ink tracking-wide mt-1">NPR {bookingAmount.toLocaleString()}</p>
                </div>

                <div class="bg-ink/[0.02] rounded-xl p-4 space-y-2">
                  {#if paymentMethod === 'esewa'}
                    <p class="text-[14px] text-ink/40">1. You will be redirected to eSewa</p>
                    <p class="text-[14px] text-ink/40">2. Login and confirm the payment</p>
                    <p class="text-[14px] text-ink/40">3. Click "Verify Payment" after completing</p>
                  {:else}
                    <p class="text-[14px] text-ink/40">1. Khalti payment widget will open</p>
                    <p class="text-[14px] text-ink/40">2. Enter your Khalti PIN to pay</p>
                    <p class="text-[14px] text-ink/40">3. Click "Verify Payment" after completing</p>
                  {/if}
                </div>

                {#if bookingError}
                  <div class="p-4 bg-crimson/5 border border-crimson/15 rounded-xl text-[14px] text-crimson font-medium">
                    {bookingError}
                  </div>
                {/if}

                <button
                  onclick={handlePaymentVerify}
                  disabled={paymentVerifying}
                  class="w-full py-3.5 text-[15px] font-semibold rounded-xl transition-all active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed
                    {paymentMethod === 'esewa' ? 'bg-green-600 text-white hover:bg-green-700' : 'bg-purple-600 text-white hover:bg-purple-700'}"
                >
                  {#if paymentVerifying}
                    <span class="inline-flex items-center gap-2">
                      <Loader2 class="w-5 h-5 animate-spin" />
                      Verifying...
                    </span>
                  {:else}
                    Verify Payment
                  {/if}
                </button>

                <button
                  onclick={() => { paymentStep = false; bookingSuccess = true; }}
                  class="w-full py-2 text-[14px] text-ink/25 hover:text-ink/50 transition-colors font-medium"
                >
                  Skip (pay later)
                </button>
              </div>

            <!-- Booking Form -->
            {:else}
              <!-- Pricing -->
              <div class="mb-6">
                <h3 class="text-[13px] font-mono uppercase tracking-wider text-ink/30 mb-4">Pricing</h3>
                <div class="space-y-3">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2 text-[15px] text-ink/40">
                      <Clock class="w-4 h-4" />
                      Per Hour
                    </div>
                    <span class="font-display text-2xl text-ink tracking-wide">NPR {vehicle.hourly_rate?.toLocaleString()}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2 text-[15px] text-ink/40">
                      <Calendar class="w-4 h-4" />
                      Per Day
                    </div>
                    <span class="font-display text-2xl text-ink tracking-wide">NPR {vehicle.daily_rate?.toLocaleString()}</span>
                  </div>
                </div>
              </div>

              <hr class="border-ink/[0.06] mb-6" />

              <!-- Booking Form -->
              <div class="space-y-4">
                <div>
                  <label for="detail-start" class="block text-[13px] font-semibold text-ink/45 mb-2">Start Date</label>
                  <input
                    id="detail-start"
                    type="date"
                    bind:value={startDate}
                    min={new Date().toISOString().split('T')[0]}
                    class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all"
                  />
                </div>
                <div>
                  <label for="detail-end" class="block text-[13px] font-semibold text-ink/45 mb-2">End Date</label>
                  <input
                    id="detail-end"
                    type="date"
                    bind:value={endDate}
                    min={startDate || new Date().toISOString().split('T')[0]}
                    class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all"
                  />
                </div>

                <!-- Payment Method -->
                <div>
                  <label class="block text-[13px] font-semibold text-ink/45 mb-2">Payment Method</label>
                  <div class="grid grid-cols-3 gap-3">
                    <button
                      type="button"
                      onclick={() => paymentMethod = 'esewa'}
                      class="flex flex-col items-center gap-1.5 p-3.5 rounded-xl border-2 text-[13px] font-semibold transition-all
                        {paymentMethod === 'esewa' ? 'border-green-500 bg-green-50 text-green-700' : 'border-ink/[0.04] bg-paper text-ink/40 hover:border-ink/10'}"
                    >
                      <CreditCard class="w-5 h-5" />
                      eSewa
                    </button>
                    <button
                      type="button"
                      onclick={() => paymentMethod = 'khalti'}
                      class="flex flex-col items-center gap-1.5 p-3.5 rounded-xl border-2 text-[13px] font-semibold transition-all
                        {paymentMethod === 'khalti' ? 'border-purple-500 bg-purple-50 text-purple-700' : 'border-ink/[0.04] bg-paper text-ink/40 hover:border-ink/10'}"
                    >
                      <Banknote class="w-5 h-5" />
                      Khalti
                    </button>
                    <button
                      type="button"
                      onclick={() => paymentMethod = 'cash'}
                      class="flex flex-col items-center gap-1.5 p-3.5 rounded-xl border-2 text-[13px] font-semibold transition-all
                        {paymentMethod === 'cash' ? 'border-saffron bg-saffron/10 text-saffron' : 'border-ink/[0.04] bg-paper text-ink/40 hover:border-ink/10'}"
                    >
                      <Wallet class="w-5 h-5" />
                      Cash
                    </button>
                  </div>
                </div>

                <!-- Total Calculation -->
                {#if totalDays > 0}
                  <div class="bg-sage/[0.05] border border-sage/10 rounded-xl p-5">
                    <div class="flex items-center justify-between text-[14px] text-ink/40 mb-2">
                      <span>Duration</span>
                      <span class="font-semibold">{totalDays} day{totalDays > 1 ? 's' : ''}</span>
                    </div>
                    <div class="flex items-center justify-between">
                      <span class="text-[14px] font-semibold text-ink/50">Total</span>
                      <span class="font-display text-3xl text-sage tracking-wide">NPR {totalAmount.toLocaleString()}</span>
                    </div>
                  </div>
                {/if}

                {#if bookingError}
                  <div class="p-4 bg-crimson/5 border border-crimson/15 rounded-xl text-[14px] text-crimson font-medium">
                    {bookingError}
                  </div>
                {/if}

                <button
                  onclick={handleBooking}
                  disabled={booking || !vehicle.available}
                  class="w-full py-3.5 bg-ink text-paper text-[15px] font-semibold rounded-xl hover:bg-ink/90 active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                >
                  {#if booking}
                    <span class="inline-flex items-center gap-2">
                      <Loader2 class="w-5 h-5 animate-spin" />
                      Booking...
                    </span>
                  {:else if !vehicle.available}
                    Currently Unavailable
                  {:else}
                    Book Now — {paymentMethod === 'cash' ? 'Pay at Pickup' : `Pay via ${paymentMethod === 'esewa' ? 'eSewa' : 'Khalti'}`}
                  {/if}
                </button>
              </div>
            {/if}
          </div>
        </div>
      </div>
    {:else}
      <!-- Vehicle Not Found -->
      <div class="text-center py-24">
        <div class="inline-flex items-center justify-center w-20 h-20 bg-ink/[0.03] rounded-2xl mb-6">
          <Car class="w-10 h-10 text-ink/15" />
        </div>
        <h2 class="font-display text-3xl text-ink tracking-wide mb-2">VEHICLE NOT FOUND</h2>
        <p class="text-[15px] text-ink/35 mb-8">This vehicle may have been removed or is no longer available.</p>
        <a
          href="/"
          class="inline-flex items-center gap-2 px-6 py-3 bg-ink text-paper text-[15px] font-semibold rounded-xl hover:bg-ink/90 transition-colors"
        >
          Browse Vehicles
        </a>
      </div>
    {/if}
  </div>
</div>
