<script lang="ts">
  import { goto } from '$app/navigation';
  import { loginWithPassword, verifyAdminOtp, registerAccount, verifyRegistration, requestPasswordReset, verifyPasswordReset } from '$lib/api';
  import { login, isAuthenticated } from '$lib/stores/auth';
  import { Phone, ArrowRight, ArrowLeft, Loader2, User, Car, Truck, Shield, Eye, EyeOff, Fingerprint, Lock } from 'lucide-svelte';

  type Mode = 'select-role' | 'login' | 'admin-otp' | 'signup' | 'signup-otp' | 'reset' | 'reset-otp';

  let mode = $state<Mode>('select-role');
  let selectedRole = $state('');
  let isSignup = $state(false);


  let phone = $state('');
  let password = $state('');
  let name = $state('');
  let otp = $state('');
  let newPassword = $state('');
  let showPassword = $state(false);


  let loading = $state(false);
  let error = $state('');
  let devOtp = $state('');

  $effect(() => {
    let unsubscribe = isAuthenticated.subscribe((val) => {
      if (val) {
        const u = typeof window !== 'undefined' ? localStorage.getItem('yatrasathi_user') : null;
        if (u) {
          const parsed = JSON.parse(u);
          if (parsed.role === 'admin') goto('/admin');
          else goto('/');
        } else goto('/');
      }
    });
    return unsubscribe;
  });

  const roles = [
    { id: 'consumer', label: 'Consumer', desc: 'Book and rent vehicles for your trips', icon: User, gradient: 'from-sage/20 to-sage/5', accent: 'sage' },
    { id: 'owner', label: 'Vehicle Owner', desc: 'List your vehicles and earn income', icon: Car, gradient: 'from-saffron/20 to-saffron/5', accent: 'saffron' },
    { id: 'driver', label: 'Driver', desc: 'Drive for riders across the valley', icon: Truck, gradient: 'from-ink/10 to-ink/5', accent: 'ink' },
    { id: 'admin', label: 'Admin', desc: 'Manage the entire platform', icon: Shield, gradient: 'from-crimson/15 to-crimson/5', accent: 'crimson' },
  ];

  const signupRoles = roles.filter(r => r.id !== 'admin');

  function selectRole(roleId: string) {
    selectedRole = roleId;
    error = '';
    devOtp = '';
    if (isSignup) {
      mode = 'signup';
    } else {
      mode = 'login';
    }
  }

  function goBack() {
    error = '';
    devOtp = '';
    if (mode === 'login' || mode === 'signup') {
      mode = 'select-role';
      phone = ''; password = ''; name = '';
    } else if (mode === 'admin-otp') {
      mode = 'login';
      otp = '';
    } else if (mode === 'signup-otp') {
      mode = 'signup';
      otp = '';
    } else if (mode === 'reset') {
      mode = 'login';
      phone = '';
    } else if (mode === 'reset-otp') {
      mode = 'reset';
      otp = ''; newPassword = '';
    }
  }

  function switchToSignup() {
    isSignup = true;
    mode = 'select-role';
    error = ''; phone = ''; password = ''; name = '';
  }

  function switchToLogin() {
    isSignup = false;
    mode = 'select-role';
    error = ''; phone = ''; password = ''; name = '';
  }


  async function handleLogin() {
    if (!phone || phone.length < 10) { error = 'Enter a valid phone number'; return; }
    if (!password || password.length < 4) { error = 'Enter your password'; return; }
    loading = true; error = '';

    try {
      const res = await loginWithPassword(phone.replace(/^\+977/, ''), password);
      if (res.error) { error = res.error; }
      else if (res.requires_otp) {
        if (res.otp) devOtp = res.otp;
        mode = 'admin-otp';
      } else if (res.token && res.user) {
        login(res.token, res.user);
      } else { error = 'Unexpected response'; }
    } catch { error = 'Login failed. Is the server running?'; }
    finally { loading = false; }
  }


  async function handleAdminOtp() {
    if (!otp || otp.length < 4) { error = 'Enter the OTP'; return; }
    loading = true; error = '';

    try {
      const res = await verifyAdminOtp(phone.replace(/^\+977/, ''), otp);
      if (res.error) { error = res.error; }
      else if (res.token && res.user) { login(res.token, res.user); }
      else { error = 'Verification failed'; }
    } catch { error = 'Verification failed'; }
    finally { loading = false; }
  }


  async function handleRegister() {
    if (!name || name.length < 2) { error = 'Enter your full name'; return; }
    if (!phone || phone.length < 10) { error = 'Enter a valid phone number'; return; }
    if (!password || password.length < 6) { error = 'Password must be at least 6 characters'; return; }
    loading = true; error = '';

    try {
      const res = await registerAccount({ phone: phone.replace(/^\+977/, ''), password, name, role: selectedRole });
      if (res.error) { error = res.error; }
      else {
        if (res.otp) devOtp = res.otp;
        mode = 'signup-otp';
      }
    } catch { error = 'Registration failed. Is the server running?'; }
    finally { loading = false; }
  }


  async function handleVerifyRegistration() {
    if (!otp || otp.length < 4) { error = 'Enter the OTP'; return; }
    loading = true; error = '';

    try {
      const res = await verifyRegistration({ phone: phone.replace(/^\+977/, ''), otp, name, password, role: selectedRole });
      if (res.error) { error = res.error; }
      else if (res.token && res.user) { login(res.token, res.user); }
      else { error = 'Verification failed'; }
    } catch { error = 'Verification failed'; }
    finally { loading = false; }
  }


  async function handleResetRequest() {
    if (!phone || phone.length < 10) { error = 'Enter your phone number'; return; }
    loading = true; error = '';

    try {
      const res = await requestPasswordReset(phone.replace(/^\+977/, ''));
      if (res.error) { error = res.error; }
      else {
        if (res.otp) devOtp = res.otp;
        mode = 'reset-otp';
      }
    } catch { error = 'Failed. Is the server running?'; }
    finally { loading = false; }
  }

  async function handleResetVerify() {
    if (!otp || otp.length < 4) { error = 'Enter the OTP'; return; }
    if (!newPassword || newPassword.length < 6) { error = 'Password must be at least 6 characters'; return; }
    loading = true; error = '';

    try {
      const res = await verifyPasswordReset(phone.replace(/^\+977/, ''), otp, newPassword);
      if (res.error) { error = res.error; }
      else {
        error = '';
        password = newPassword;
        mode = 'login';
      }
    } catch { error = 'Reset failed'; }
    finally { loading = false; }
  }

  function getRoleLabel(id: string) {
    return roles.find(r => r.id === id)?.label || id;
  }
</script>

<svelte:head>
  <title>{isSignup ? 'Sign Up' : 'Login'} - YatraSathi</title>
</svelte:head>

<div class="min-h-screen pt-14 flex items-center justify-center bg-ink px-4 py-16 relative overflow-hidden">

  <div class="absolute inset-0 overflow-hidden pointer-events-none">
    <div class="absolute top-1/4 -left-32 w-96 h-96 bg-sage/[0.03] rounded-full blur-3xl"></div>
    <div class="absolute bottom-1/4 -right-32 w-96 h-96 bg-saffron/[0.03] rounded-full blur-3xl"></div>
  </div>

  <div class="relative w-full max-w-md z-10">

    <div class="text-center mb-10">
      <div class="flex items-center justify-center gap-2 mb-3">
        <span class="font-display text-4xl tracking-wide text-paper">YATRA</span>
        <span class="font-display text-4xl tracking-wide text-saffron">SATHI</span>
      </div>
      <p class="text-[15px] text-paper/40">
        {#if isSignup}Create your account to get started{:else}Welcome back, sign in to continue{/if}
      </p>
    </div>


    <div class="bg-white rounded-2xl p-8 shadow-xl shadow-black/20">


      {#if mode === 'select-role'}
        <div class="text-center mb-6">
          <h2 class="font-display text-2xl text-ink tracking-wide">
            {isSignup ? 'I WANT TO SIGN UP AS' : 'I AM A'}
          </h2>
          <p class="text-[14px] text-ink/40 mt-1">Select your role to continue</p>
        </div>

        <div class="grid grid-cols-2 gap-4">
          {#each (isSignup ? signupRoles : roles) as role}
            <button
              onclick={() => selectRole(role.id)}
              class="group relative p-5 rounded-2xl border-2 border-ink/[0.04] hover:border-ink/10 hover:shadow-md transition-all text-left overflow-hidden"
            >
              <div class="absolute inset-0 bg-gradient-to-br {role.gradient} opacity-0 group-hover:opacity-100 transition-opacity"></div>
              <div class="relative">
                <div class="w-10 h-10 rounded-xl bg-gradient-to-br {role.gradient} flex items-center justify-center mb-3">
                  <role.icon class="w-5 h-5 text-ink/40 group-hover:text-ink/70 transition-colors" />
                </div>
                <p class="text-[15px] font-semibold text-ink">{role.label}</p>
                <p class="text-[13px] text-ink/35 mt-1 leading-snug">{role.desc}</p>
              </div>
            </button>
          {/each}
        </div>

        <div class="mt-8 text-center">
          {#if isSignup}
            <p class="text-[14px] text-ink/40">
              Already have an account?
              <button onclick={switchToLogin} class="text-sage font-semibold hover:underline underline-offset-2 ml-1">Sign in</button>
            </p>
          {:else}
            <p class="text-[14px] text-ink/40">
              Don't have an account?
              <button onclick={switchToSignup} class="text-sage font-semibold hover:underline underline-offset-2 ml-1">Create one</button>
            </p>
          {/if}
        </div>


      {:else if mode === 'login'}
        <div class="flex items-center gap-3 mb-6">
          <button onclick={goBack} class="p-2 hover:bg-paper rounded-xl transition-colors">
            <ArrowLeft class="w-5 h-5 text-ink/30" />
          </button>
          <div>
            <h2 class="font-display text-2xl text-ink tracking-wide">SIGN IN</h2>
            <p class="text-[14px] text-ink/40 mt-0.5">Login as {getRoleLabel(selectedRole)}</p>
          </div>
        </div>

        <div class="space-y-5">
          <div>
            <label for="login-phone" class="block text-[13px] font-semibold text-ink/50 mb-2">Phone Number</label>
            <div class="flex gap-3">
              <div class="flex items-center px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] font-mono text-ink/40 shrink-0">
                +977
              </div>
              <input id="login-phone" type="tel" bind:value={phone} placeholder="98XXXXXXXX" maxlength="10"
                class="flex-1 px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all placeholder:text-ink/20"
                onkeydown={(e) => { if (e.key === 'Enter') document.getElementById('login-pw')?.focus(); }} />
            </div>
          </div>

          <div>
            <label for="login-pw" class="block text-[13px] font-semibold text-ink/50 mb-2">Password</label>
            <div class="relative">
              <input id="login-pw" type={showPassword ? 'text' : 'password'} bind:value={password} placeholder="Enter your password"
                class="w-full px-4 py-3 pr-12 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all placeholder:text-ink/20"
                onkeydown={(e) => { if (e.key === 'Enter') handleLogin(); }} />
              <button type="button" onclick={() => showPassword = !showPassword}
                class="absolute right-3.5 top-1/2 -translate-y-1/2 p-1 text-ink/25 hover:text-ink/50 transition-colors">
                {#if showPassword}<EyeOff class="w-5 h-5" />{:else}<Eye class="w-5 h-5" />{/if}
              </button>
            </div>
          </div>

          {#if error}
            <div class="p-4 bg-crimson/5 border border-crimson/15 rounded-xl text-[14px] text-crimson font-medium">{error}</div>
          {/if}

          <button onclick={handleLogin} disabled={loading}
            class="w-full flex items-center justify-center gap-2.5 py-3.5 bg-ink text-paper text-[15px] font-semibold rounded-xl hover:bg-ink/90 active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed transition-all">
            {#if loading}<Loader2 class="w-5 h-5 animate-spin" /> Signing in...{:else}Sign In <ArrowRight class="w-5 h-5" />{/if}
          </button>

          <div class="flex items-center justify-between pt-1">
            <button onclick={() => { mode = 'reset'; error = ''; }} class="text-[14px] text-ink/30 hover:text-sage transition-colors font-medium">
              Forgot password?
            </button>
            <button onclick={switchToSignup} class="text-[14px] text-sage font-semibold hover:underline underline-offset-2">
              Create account
            </button>
          </div>
        </div>


      {:else if mode === 'admin-otp'}
        <div class="flex items-center gap-3 mb-6">
          <button onclick={goBack} class="p-2 hover:bg-paper rounded-xl transition-colors">
            <ArrowLeft class="w-5 h-5 text-ink/30" />
          </button>
          <div>
            <h2 class="font-display text-2xl text-ink tracking-wide">VERIFICATION</h2>
            <p class="text-[14px] text-ink/40 mt-0.5">Admin two-factor authentication</p>
          </div>
        </div>

        <div class="text-center mb-6">
          <div class="inline-flex items-center justify-center w-14 h-14 rounded-2xl bg-gradient-to-br from-crimson/15 to-crimson/5 mb-3">
            <Fingerprint class="w-7 h-7 text-crimson/70" />
          </div>
          <p class="text-[14px] text-ink/40">Enter the OTP sent to <span class="font-semibold text-ink/60">+977 {phone}</span></p>
        </div>

        {#if devOtp}
          <div class="mb-5 p-4 bg-sage/8 border border-sage/15 rounded-xl text-[14px] text-sage">
            <span class="font-semibold">Dev Mode:</span> OTP is <span class="font-mono font-bold text-[16px]">{devOtp}</span>
          </div>
        {/if}

        <div class="space-y-5">
          <div>
            <label for="admin-otp" class="block text-[13px] font-semibold text-ink/50 mb-2">OTP Code</label>
            <input id="admin-otp" type="text" bind:value={otp} placeholder="Enter 6-digit OTP" maxlength="6"
              class="w-full px-4 py-4 bg-paper border border-ink/[0.06] rounded-xl text-center text-xl font-mono tracking-[0.4em] focus:outline-none focus:border-crimson/40 focus:ring-2 focus:ring-crimson/10 transition-all placeholder:text-ink/20 placeholder:tracking-normal placeholder:text-[15px]"
              onkeydown={(e) => { if (e.key === 'Enter') handleAdminOtp(); }} />
          </div>

          {#if error}
            <div class="p-4 bg-crimson/5 border border-crimson/15 rounded-xl text-[14px] text-crimson font-medium">{error}</div>
          {/if}

          <button onclick={handleAdminOtp} disabled={loading}
            class="w-full flex items-center justify-center gap-2.5 py-3.5 bg-crimson text-white text-[15px] font-semibold rounded-xl hover:bg-crimson/90 active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed transition-all">
            {#if loading}<Loader2 class="w-5 h-5 animate-spin" /> Verifying...{:else}Verify & Sign In{/if}
          </button>
        </div>


      {:else if mode === 'signup'}
        <div class="flex items-center gap-3 mb-6">
          <button onclick={goBack} class="p-2 hover:bg-paper rounded-xl transition-colors">
            <ArrowLeft class="w-5 h-5 text-ink/30" />
          </button>
          <div>
            <h2 class="font-display text-2xl text-ink tracking-wide">CREATE ACCOUNT</h2>
            <p class="text-[14px] text-ink/40 mt-0.5">Sign up as {getRoleLabel(selectedRole)}</p>
          </div>
        </div>

        <div class="space-y-5">
          <div>
            <label for="signup-name" class="block text-[13px] font-semibold text-ink/50 mb-2">Full Name</label>
            <input id="signup-name" type="text" bind:value={name} placeholder="Your full name"
              class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all placeholder:text-ink/20" />
          </div>

          <div>
            <label for="signup-phone" class="block text-[13px] font-semibold text-ink/50 mb-2">Phone Number</label>
            <div class="flex gap-3">
              <div class="flex items-center px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] font-mono text-ink/40 shrink-0">
                +977
              </div>
              <input id="signup-phone" type="tel" bind:value={phone} placeholder="98XXXXXXXX" maxlength="10"
                class="flex-1 px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all placeholder:text-ink/20" />
            </div>
          </div>

          <div>
            <label for="signup-pw" class="block text-[13px] font-semibold text-ink/50 mb-2">Password</label>
            <div class="relative">
              <input id="signup-pw" type={showPassword ? 'text' : 'password'} bind:value={password} placeholder="Minimum 6 characters"
                class="w-full px-4 py-3 pr-12 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all placeholder:text-ink/20" />
              <button type="button" onclick={() => showPassword = !showPassword}
                class="absolute right-3.5 top-1/2 -translate-y-1/2 p-1 text-ink/25 hover:text-ink/50 transition-colors">
                {#if showPassword}<EyeOff class="w-5 h-5" />{:else}<Eye class="w-5 h-5" />{/if}
              </button>
            </div>
            <p class="text-[13px] text-ink/25 mt-1.5">Must be at least 6 characters</p>
          </div>

          {#if error}
            <div class="p-4 bg-crimson/5 border border-crimson/15 rounded-xl text-[14px] text-crimson font-medium">{error}</div>
          {/if}

          <button onclick={handleRegister} disabled={loading}
            class="w-full flex items-center justify-center gap-2.5 py-3.5 bg-ink text-paper text-[15px] font-semibold rounded-xl hover:bg-ink/90 active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed transition-all">
            {#if loading}<Loader2 class="w-5 h-5 animate-spin" /> Sending OTP...{:else}Continue <ArrowRight class="w-5 h-5" />{/if}
          </button>

          <p class="text-[14px] text-ink/35 text-center">
            Already have an account? <button onclick={switchToLogin} class="text-sage font-semibold hover:underline underline-offset-2">Sign in</button>
          </p>
        </div>


      {:else if mode === 'signup-otp'}
        <div class="flex items-center gap-3 mb-6">
          <button onclick={goBack} class="p-2 hover:bg-paper rounded-xl transition-colors">
            <ArrowLeft class="w-5 h-5 text-ink/30" />
          </button>
          <div>
            <h2 class="font-display text-2xl text-ink tracking-wide">VERIFY PHONE</h2>
            <p class="text-[14px] text-ink/40 mt-0.5">Almost there! Confirm your number</p>
          </div>
        </div>

        <div class="text-center mb-6">
          <div class="inline-flex items-center justify-center w-14 h-14 rounded-2xl bg-gradient-to-br from-sage/20 to-sage/5 mb-3">
            <Phone class="w-7 h-7 text-sage/70" />
          </div>
          <p class="text-[14px] text-ink/40">OTP sent to <span class="font-semibold text-ink/60">+977 {phone}</span></p>
        </div>

        {#if devOtp}
          <div class="mb-5 p-4 bg-sage/8 border border-sage/15 rounded-xl text-[14px] text-sage">
            <span class="font-semibold">Dev Mode:</span> OTP is <span class="font-mono font-bold text-[16px]">{devOtp}</span>
          </div>
        {/if}

        <div class="space-y-5">
          <div>
            <label for="verify-otp" class="block text-[13px] font-semibold text-ink/50 mb-2">Verification Code</label>
            <input id="verify-otp" type="text" bind:value={otp} placeholder="Enter 6-digit OTP" maxlength="6"
              class="w-full px-4 py-4 bg-paper border border-ink/[0.06] rounded-xl text-center text-xl font-mono tracking-[0.4em] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all placeholder:text-ink/20 placeholder:tracking-normal placeholder:text-[15px]"
              onkeydown={(e) => { if (e.key === 'Enter') handleVerifyRegistration(); }} />
          </div>

          {#if error}
            <div class="p-4 bg-crimson/5 border border-crimson/15 rounded-xl text-[14px] text-crimson font-medium">{error}</div>
          {/if}

          <button onclick={handleVerifyRegistration} disabled={loading}
            class="w-full flex items-center justify-center gap-2.5 py-3.5 bg-sage text-white text-[15px] font-semibold rounded-xl hover:bg-sage/90 active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed transition-all">
            {#if loading}<Loader2 class="w-5 h-5 animate-spin" /> Creating account...{:else}Verify & Create Account{/if}
          </button>
        </div>


      {:else if mode === 'reset'}
        <div class="flex items-center gap-3 mb-6">
          <button onclick={goBack} class="p-2 hover:bg-paper rounded-xl transition-colors">
            <ArrowLeft class="w-5 h-5 text-ink/30" />
          </button>
          <div>
            <h2 class="font-display text-2xl text-ink tracking-wide">RESET PASSWORD</h2>
            <p class="text-[14px] text-ink/40 mt-0.5">We'll send a verification code</p>
          </div>
        </div>

        <div class="text-center mb-6">
          <div class="inline-flex items-center justify-center w-14 h-14 rounded-2xl bg-gradient-to-br from-saffron/20 to-saffron/5 mb-3">
            <Lock class="w-7 h-7 text-saffron/70" />
          </div>
        </div>

        <div class="space-y-5">
          <div>
            <label for="reset-phone" class="block text-[13px] font-semibold text-ink/50 mb-2">Phone Number</label>
            <div class="flex gap-3">
              <div class="flex items-center px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] font-mono text-ink/40 shrink-0">
                +977
              </div>
              <input id="reset-phone" type="tel" bind:value={phone} placeholder="98XXXXXXXX" maxlength="10"
                class="flex-1 px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all placeholder:text-ink/20"
                onkeydown={(e) => { if (e.key === 'Enter') handleResetRequest(); }} />
            </div>
          </div>

          {#if error}
            <div class="p-4 bg-crimson/5 border border-crimson/15 rounded-xl text-[14px] text-crimson font-medium">{error}</div>
          {/if}

          <button onclick={handleResetRequest} disabled={loading}
            class="w-full flex items-center justify-center gap-2.5 py-3.5 bg-ink text-paper text-[15px] font-semibold rounded-xl hover:bg-ink/90 active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed transition-all">
            {#if loading}<Loader2 class="w-5 h-5 animate-spin" /> Sending...{:else}Send Verification Code{/if}
          </button>
        </div>


      {:else if mode === 'reset-otp'}
        <div class="flex items-center gap-3 mb-6">
          <button onclick={goBack} class="p-2 hover:bg-paper rounded-xl transition-colors">
            <ArrowLeft class="w-5 h-5 text-ink/30" />
          </button>
          <div>
            <h2 class="font-display text-2xl text-ink tracking-wide">NEW PASSWORD</h2>
            <p class="text-[14px] text-ink/40 mt-0.5">Set your new password</p>
          </div>
        </div>

        {#if devOtp}
          <div class="mb-5 p-4 bg-sage/8 border border-sage/15 rounded-xl text-[14px] text-sage">
            <span class="font-semibold">Dev Mode:</span> OTP is <span class="font-mono font-bold text-[16px]">{devOtp}</span>
          </div>
        {/if}

        <div class="space-y-5">
          <div>
            <label for="reset-otp" class="block text-[13px] font-semibold text-ink/50 mb-2">OTP Code</label>
            <input id="reset-otp" type="text" bind:value={otp} placeholder="Enter 6-digit OTP" maxlength="6"
              class="w-full px-4 py-4 bg-paper border border-ink/[0.06] rounded-xl text-center text-xl font-mono tracking-[0.4em] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all placeholder:text-ink/20 placeholder:tracking-normal placeholder:text-[15px]" />
          </div>

          <div>
            <label for="new-pw" class="block text-[13px] font-semibold text-ink/50 mb-2">New Password</label>
            <input id="new-pw" type="password" bind:value={newPassword} placeholder="Minimum 6 characters"
              class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 transition-all placeholder:text-ink/20"
              onkeydown={(e) => { if (e.key === 'Enter') handleResetVerify(); }} />
          </div>

          {#if error}
            <div class="p-4 bg-crimson/5 border border-crimson/15 rounded-xl text-[14px] text-crimson font-medium">{error}</div>
          {/if}

          <button onclick={handleResetVerify} disabled={loading}
            class="w-full flex items-center justify-center gap-2.5 py-3.5 bg-ink text-paper text-[15px] font-semibold rounded-xl hover:bg-ink/90 active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed transition-all">
            {#if loading}<Loader2 class="w-5 h-5 animate-spin" /> Resetting...{:else}Reset Password{/if}
          </button>
        </div>
      {/if}
    </div>

    <p class="text-center text-[13px] text-paper/20 mt-6">
      By continuing, you agree to YatraSathi's Terms of Service
    </p>
  </div>
</div>
