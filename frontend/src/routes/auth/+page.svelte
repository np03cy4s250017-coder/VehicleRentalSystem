<script lang="ts">
  import { goto } from '$app/navigation';
  import { sendOtp, verifyOtp } from '$lib/api';
  import { login, isAuthenticated } from '$lib/stores/auth';
  import { Phone, KeyRound, ArrowRight, Zap, Loader2 } from 'lucide-svelte';

  let step = $state<'phone' | 'otp'>('phone');
  let phone = $state('');
  let otp = $state('');
  let loading = $state(false);
  let error = $state('');
  let devOtp = $state('');

  // Redirect if already logged in
  $effect(() => {
    let unsubscribe = isAuthenticated.subscribe((val) => {
      if (val) goto('/');
    });
    return unsubscribe;
  });

  async function handleSendOtp() {
    if (!phone || phone.length < 10) {
      error = 'Please enter a valid phone number';
      return;
    }

    loading = true;
    error = '';

    try {
      const res = await sendOtp(phone.replace(/^\+977/, ''));
      if (res.error) {
        error = res.error || res.message || 'Failed to send OTP';
      } else {
        step = 'otp';
        // Show OTP in dev mode
        if (res.otp) {
          devOtp = res.otp;
        }
      }
    } catch {
      error = 'Failed to send OTP. Is the server running?';
    } finally {
      loading = false;
    }
  }

  async function handleVerifyOtp() {
    if (!otp || otp.length < 4) {
      error = 'Please enter a valid OTP';
      return;
    }

    loading = true;
    error = '';

    try {
      const res = await verifyOtp(phone.replace(/^\+977/, ''), otp);
      if (res.error) {
        error = res.error || res.message || 'Invalid OTP';
      } else if (res.token && res.user) {
        login(res.token, res.user);
        goto('/');
      } else {
        error = 'Unexpected response from server';
      }
    } catch {
      error = 'Failed to verify OTP. Please try again.';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Login - YatraSathi</title>
</svelte:head>

<div class="min-h-screen pt-16 flex items-center justify-center bg-gradient-to-br from-emerald-800 via-emerald-700 to-emerald-900 px-4 py-12">
  <!-- Background pattern -->
  <div class="absolute inset-0 opacity-5">
    <svg class="w-full h-full" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <pattern id="auth-grid" width="60" height="60" patternUnits="userSpaceOnUse">
          <path d="M 60 0 L 0 0 0 60" fill="none" stroke="white" stroke-width="0.5"/>
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill="url(#auth-grid)" />
    </svg>
  </div>

  <div class="relative w-full max-w-md">
    <!-- Logo -->
    <div class="text-center mb-8">
      <div class="inline-flex items-center gap-2 mb-2">
        <span class="text-3xl">🏔️</span>
        <span class="text-2xl font-bold text-white">Yatra<span class="text-amber-400">Sathi</span></span>
        <Zap class="w-5 h-5 text-sky-400" />
      </div>
      <p class="text-emerald-200 text-sm">Login to your account</p>
    </div>

    <!-- Card -->
    <div class="bg-white rounded-2xl shadow-2xl p-8">
      {#if step === 'phone'}
        <!-- Step 1: Phone -->
        <div class="text-center mb-6">
          <div class="inline-flex items-center justify-center w-14 h-14 bg-emerald-100 rounded-xl mb-4">
            <Phone class="w-7 h-7 text-emerald-600" />
          </div>
          <h2 class="text-xl font-bold text-slate-900">Enter Your Phone</h2>
          <p class="text-sm text-slate-500 mt-1">We'll send you a one-time verification code</p>
        </div>

        <div class="space-y-4">
          <div>
            <label for="phone" class="block text-sm font-medium text-slate-700 mb-1.5">Phone Number</label>
            <div class="flex gap-2">
              <div class="flex items-center px-4 py-3 bg-slate-100 border border-slate-200 rounded-xl text-sm font-medium text-slate-600 shrink-0">
                +977
              </div>
              <input
                id="phone"
                type="tel"
                bind:value={phone}
                placeholder="98XXXXXXXX"
                maxlength="10"
                class="flex-1 px-4 py-3 bg-slate-50 border border-slate-200 rounded-xl text-sm focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all"
                onkeydown={(e) => { if (e.key === 'Enter') handleSendOtp(); }}
              />
            </div>
          </div>

          {#if error}
            <div class="p-3 bg-red-50 border border-red-200 rounded-xl text-sm text-red-600">
              {error}
            </div>
          {/if}

          <button
            onclick={handleSendOtp}
            disabled={loading}
            class="w-full flex items-center justify-center gap-2 py-3 bg-emerald-600 text-white font-semibold rounded-xl hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-lg shadow-emerald-600/20"
          >
            {#if loading}
              <Loader2 class="w-5 h-5 animate-spin" />
              Sending...
            {:else}
              Send OTP
              <ArrowRight class="w-5 h-5" />
            {/if}
          </button>
        </div>
      {:else}
        <!-- Step 2: OTP -->
        <div class="text-center mb-6">
          <div class="inline-flex items-center justify-center w-14 h-14 bg-amber-100 rounded-xl mb-4">
            <KeyRound class="w-7 h-7 text-amber-600" />
          </div>
          <h2 class="text-xl font-bold text-slate-900">Enter OTP</h2>
          <p class="text-sm text-slate-500 mt-1">
            Code sent to +977 {phone}
          </p>
        </div>

        {#if devOtp}
          <div class="mb-4 p-3 bg-sky-50 border border-sky-200 rounded-xl text-sm text-sky-700">
            <span class="font-semibold">Dev Mode:</span> Your OTP is <span class="font-mono font-bold">{devOtp}</span>
          </div>
        {/if}

        <div class="space-y-4">
          <div>
            <label for="otp" class="block text-sm font-medium text-slate-700 mb-1.5">Verification Code</label>
            <input
              id="otp"
              type="text"
              bind:value={otp}
              placeholder="Enter 6-digit OTP"
              maxlength="6"
              class="w-full px-4 py-3 bg-slate-50 border border-slate-200 rounded-xl text-center text-lg font-mono tracking-widest focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 transition-all"
              onkeydown={(e) => { if (e.key === 'Enter') handleVerifyOtp(); }}
            />
          </div>

          {#if error}
            <div class="p-3 bg-red-50 border border-red-200 rounded-xl text-sm text-red-600">
              {error}
            </div>
          {/if}

          <button
            onclick={handleVerifyOtp}
            disabled={loading}
            class="w-full flex items-center justify-center gap-2 py-3 bg-emerald-600 text-white font-semibold rounded-xl hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-lg shadow-emerald-600/20"
          >
            {#if loading}
              <Loader2 class="w-5 h-5 animate-spin" />
              Verifying...
            {:else}
              Verify & Login
            {/if}
          </button>

          <button
            onclick={() => { step = 'phone'; error = ''; devOtp = ''; otp = ''; }}
            class="w-full py-2 text-sm text-slate-500 hover:text-slate-700 transition-colors"
          >
            Change phone number
          </button>
        </div>
      {/if}
    </div>

    <p class="text-center text-sm text-emerald-200 mt-6">
      By continuing, you agree to YatraSathi's Terms of Service
    </p>
  </div>
</div>
