<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAdmin, user } from '$lib/stores/auth';
  import {
    getAdminStats, getAdminUsers, getAdminBookings, getAdminVehicles,
    updateBookingStatus, deleteVehicle, createVehicle
  } from '$lib/api';
  import {
    Users, Car, Calendar, DollarSign, Loader2, CheckCircle,
    XCircle, Trash2, Plus, X, Eye, Zap, MapPin, Clock, AlertTriangle,
    TrendingUp, Leaf, Activity, ChevronRight, LayoutDashboard, Banknote,
    ExternalLink
  } from 'lucide-svelte';

  let stats = $state({ users: 0, vehicles: 0, bookings: 0, revenue: 0 });
  let recentBookings = $state<any[]>([]);
  let users_list = $state<any[]>([]);
  let vehicles = $state<any[]>([]);
  let bookings = $state<any[]>([]);
  let loading = $state(true);
  let activeView = $state<'overview' | 'bookings' | 'vehicles' | 'users'>('overview');
  let actionLoading = $state('');
  let actionError = $state('');
  let actionSuccess = $state('');
  let showAddVehicle = $state(false);
  let deleteConfirm = $state('');
  let addingVehicle = $state(false);
  let newFeature = $state('');

  let newVehicle = $state({
    type: 'car', make: '', model: '', year: 2024, is_ev: true,
    ev_range_km: 300, plate_no: '', hourly_rate: 1000, daily_rate: 5000,
    description: '', image_url: '', location_name: 'Kathmandu', features: [] as string[],
  });

  onMount(() => {
    const unsubscribe = isAdmin.subscribe((val) => { if (!val) goto('/'); });
    loadData();
    return unsubscribe;
  });

  async function loadData() {
    loading = true;
    try {
      const [statsRes, usersRes, vehiclesRes, bookingsRes] = await Promise.all([
        getAdminStats().catch(() => ({})),
        getAdminUsers().catch(() => ({ users: [] })),
        getAdminVehicles().catch(() => ({ vehicles: [] })),
        getAdminBookings().catch(() => ({ bookings: [] })),
      ]);
      stats = {
        users: statsRes.total_users || statsRes.users || 0,
        vehicles: statsRes.total_vehicles || statsRes.vehicles || 0,
        bookings: statsRes.total_bookings || statsRes.bookings || 0,
        revenue: statsRes.total_revenue || statsRes.revenue || 0,
      };
      recentBookings = statsRes.recent_bookings || [];
      users_list = usersRes.users || usersRes || [];
      vehicles = vehiclesRes.vehicles || vehiclesRes || [];
      bookings = bookingsRes.bookings || bookingsRes || [];
    } catch {} finally { loading = false; }
  }

  function fmt(dateStr: string) {
    if (!dateStr) return '-';
    try {
      const d = new Date(dateStr);
      if (isNaN(d.getTime())) return dateStr.split('T')[0] || dateStr;
      return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
    } catch { return dateStr; }
  }

  function fmtFull(dateStr: string) {
    if (!dateStr) return '-';
    try {
      const d = new Date(dateStr);
      if (isNaN(d.getTime())) return dateStr.split('T')[0] || dateStr;
      return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
    } catch { return dateStr; }
  }

  function clearMsg() { actionError = ''; actionSuccess = ''; }

  async function bookingAction(id: string, status: string) {
    clearMsg(); actionLoading = id;
    try {
      const res = await updateBookingStatus(id, status);
      if (res.error) { actionError = res.error; }
      else {
        actionSuccess = `Booking ${status}`;
        bookings = bookings.map(b => b.id === id ? { ...b, status } : b);
        const s = await getAdminStats().catch(() => null);
        if (s) stats = {
          users: s.total_users || s.users || stats.users,
          vehicles: s.total_vehicles || s.vehicles || stats.vehicles,
          bookings: s.total_bookings || s.bookings || stats.bookings,
          revenue: s.total_revenue || s.revenue || stats.revenue,
        };
      }
    } catch { actionError = 'Failed to update'; }
    finally { actionLoading = ''; setTimeout(() => actionSuccess = '', 3000); }
  }

  async function removeVehicle(id: string) {
    clearMsg(); actionLoading = id;
    try {
      const res = await deleteVehicle(id);
      if (res.error) actionError = res.error;
      else {
        actionSuccess = 'Vehicle removed';
        vehicles = vehicles.filter(v => v.id !== id);
        stats.vehicles = vehicles.length;
      }
    } catch { actionError = 'Failed'; }
    finally { actionLoading = ''; deleteConfirm = ''; setTimeout(() => actionSuccess = '', 3000); }
  }

  async function addVehicle() {
    clearMsg(); addingVehicle = true;
    try {
      const res = await createVehicle(newVehicle);
      if (res.error) actionError = res.error;
      else {
        actionSuccess = 'Vehicle added successfully';
        showAddVehicle = false;
        newVehicle = {
          type: 'car', make: '', model: '', year: 2024, is_ev: true,
          ev_range_km: 300, plate_no: '', hourly_rate: 1000, daily_rate: 5000,
          description: '', image_url: '', location_name: 'Kathmandu', features: [],
        };
        await loadData();
      }
    } catch { actionError = 'Failed'; }
    finally { addingVehicle = false; setTimeout(() => actionSuccess = '', 3000); }
  }

  function addFeat() {
    if (newFeature.trim() && !newVehicle.features.includes(newFeature.trim())) {
      newVehicle.features = [...newVehicle.features, newFeature.trim()];
      newFeature = '';
    }
  }
  function remFeat(f: string) {
    newVehicle.features = newVehicle.features.filter(x => x !== f);
  }

  let pending = $derived(bookings.filter(b => b.status === 'pending'));
  let activeCount = $derived(bookings.filter(b => b.status === 'active' || b.status === 'confirmed').length);
  let completedCount = $derived(bookings.filter(b => b.status === 'completed').length);
  let evCount = $derived(vehicles.filter((v: any) => v.is_ev).length);
  let totalCarbon = $derived(bookings.reduce((s: number, b: any) => s + (b.carbon_saved_kg || 0), 0));

  const stBg: Record<string, string> = {
    pending: 'bg-saffron/10 text-saffron border-saffron/20',
    confirmed: 'bg-sage/10 text-sage border-sage/20',
    active: 'bg-sage/10 text-sage border-sage/20',
    completed: 'bg-ink/5 text-ink/40 border-ink/10',
    cancelled: 'bg-crimson/10 text-crimson border-crimson/20',
  };

  const stDot: Record<string, string> = {
    pending: 'bg-saffron',
    confirmed: 'bg-sage',
    active: 'bg-sage',
    completed: 'bg-ink/25',
    cancelled: 'bg-crimson',
  };

  const roleBg: Record<string, string> = {
    consumer: 'bg-ink/5 text-ink/50',
    renter: 'bg-ink/5 text-ink/50',
    owner: 'bg-sage/10 text-sage',
    driver: 'bg-saffron/10 text-saffron',
    admin: 'bg-crimson/10 text-crimson',
  };

  const payLbl: Record<string, string> = {
    esewa: 'eSewa',
    khalti: 'Khalti',
    cash: 'Cash',
    connectips: 'ConnectIPS',
  };

  const nav = [
    { id: 'overview' as const, label: 'Overview', icon: LayoutDashboard },
    { id: 'bookings' as const, label: 'Bookings', icon: Calendar },
    { id: 'vehicles' as const, label: 'Vehicles', icon: Car },
    { id: 'users' as const, label: 'Users', icon: Users },
  ];
</script>

<svelte:head>
  <title>Admin Dashboard - YatraSathi</title>
</svelte:head>

<div class="min-h-screen bg-paper">


  <div class="bg-white border-b border-ink/[0.04] sticky top-16 z-30">
    <div class="max-w-7xl mx-auto px-6 lg:px-8 flex items-center justify-between">
      <div class="flex items-center gap-0.5">
        {#each nav as item}
          <button
            onclick={() => activeView = item.id}
            class="relative flex items-center gap-2.5 px-5 py-4 text-[14px] font-semibold transition-colors
              {activeView === item.id ? 'text-ink' : 'text-ink/30 hover:text-ink/60'}"
          >
            <item.icon class="w-[18px] h-[18px]" />
            <span class="hidden sm:inline">{item.label}</span>
            {#if item.id === 'bookings' && pending.length > 0}
              <span class="w-5 h-5 bg-crimson text-white text-[11px] font-bold rounded-full flex items-center justify-center">
                {pending.length}
              </span>
            {/if}
            {#if activeView === item.id}
              <div class="absolute bottom-0 left-3 right-3 h-[2px] bg-ink rounded-full"></div>
            {/if}
          </button>
        {/each}
      </div>
      <div class="flex items-center gap-4">
        <a href="/" class="text-[13px] text-ink/25 hover:text-ink/50 transition-colors hidden sm:flex items-center gap-1">
          View Site <ExternalLink class="w-3.5 h-3.5" />
        </a>
        {#if activeView === 'vehicles'}
          <button onclick={() => showAddVehicle = true}
            class="flex items-center gap-2 px-5 py-2.5 bg-ink text-paper text-[13px] font-semibold rounded-xl hover:bg-ink/90 active:scale-[0.98] transition-all">
            <Plus class="w-4 h-4" /> Add Vehicle
          </button>
        {/if}
      </div>
    </div>
  </div>


  {#if actionSuccess || actionError}
    <div class="max-w-7xl mx-auto px-6 lg:px-8 pt-5">
      {#if actionSuccess}
        <div class="mb-3 p-4 bg-sage/8 border border-sage/15 rounded-xl text-[14px] text-sage font-medium flex items-center gap-2">
          <CheckCircle class="w-4 h-4 shrink-0" /> {actionSuccess}
        </div>
      {/if}
      {#if actionError}
        <div class="mb-3 p-4 bg-crimson/8 border border-crimson/15 rounded-xl text-[14px] text-crimson font-medium flex items-center gap-2">
          <AlertTriangle class="w-4 h-4 shrink-0" /> {actionError}
          <button onclick={() => actionError = ''} class="ml-auto p-0.5 hover:bg-crimson/10 rounded-lg transition-colors">
            <X class="w-4 h-4" />
          </button>
        </div>
      {/if}
    </div>
  {/if}


  {#if loading}
    <div class="flex flex-col items-center justify-center py-32">
      <Loader2 class="w-8 h-8 text-sage animate-spin mb-4" />
      <p class="text-[15px] text-ink/30">Loading admin data...</p>
    </div>


  {:else if activeView === 'overview'}
    <div class="max-w-7xl mx-auto px-6 lg:px-8 py-8">


      <div class="flex items-center justify-between mb-8">
        <div>
          <p class="text-[13px] font-mono uppercase tracking-wider text-ink/25 mb-1">Welcome back</p>
          <h1 class="font-display text-5xl text-ink tracking-wide">{$user?.name || 'ADMIN'}</h1>
        </div>
        <div class="text-right hidden sm:block">
          <p class="text-[15px] text-ink/35 font-medium">YatraSathi Platform</p>
          <p class="text-[13px] text-ink/20 mt-0.5">
            {new Date().toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric' })}
          </p>
        </div>
      </div>


      <div class="grid grid-cols-2 lg:grid-cols-4 gap-5 mb-8">


        <button onclick={() => activeView = 'bookings'}
          class="group bg-white rounded-2xl p-6 border border-ink/[0.04] shadow-sm text-left hover:shadow-md hover:border-ink/[0.08] transition-all">
          <div class="flex items-center justify-between mb-5">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-saffron/20 to-saffron/5 flex items-center justify-center">
              <Calendar class="w-6 h-6 text-saffron" />
            </div>
            <ChevronRight class="w-5 h-5 text-ink/10 group-hover:text-ink/30 group-hover:translate-x-0.5 transition-all" />
          </div>
          <p class="font-display text-4xl text-ink tracking-wide">{stats.bookings}</p>
          <p class="text-[14px] text-ink/35 mt-1 font-medium">Total Bookings</p>
          {#if pending.length > 0}
            <div class="mt-3 flex items-center gap-2">
              <span class="w-2 h-2 bg-saffron rounded-full animate-pulse"></span>
              <span class="text-[13px] text-saffron font-semibold">{pending.length} awaiting approval</span>
            </div>
          {/if}
        </button>


        <div class="bg-white rounded-2xl p-6 border border-ink/[0.04] shadow-sm">
          <div class="flex items-center justify-between mb-5">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-sage/20 to-sage/5 flex items-center justify-center">
              <DollarSign class="w-6 h-6 text-sage" />
            </div>
            <TrendingUp class="w-5 h-5 text-sage/30" />
          </div>
          <p class="font-display text-4xl text-ink tracking-wide">
            NPR {stats.revenue >= 1000 ? `${(stats.revenue / 1000).toFixed(1)}K` : stats.revenue}
          </p>
          <p class="text-[14px] text-ink/35 mt-1 font-medium">Revenue Earned</p>
          <div class="mt-3 h-1.5 bg-ink/[0.04] rounded-full overflow-hidden">
            <div class="h-full bg-gradient-to-r from-sage to-sage/70 rounded-full transition-all duration-700"
              style="width: {Math.min(100, (stats.revenue / 100000) * 100)}%"></div>
          </div>
        </div>


        <button onclick={() => activeView = 'vehicles'}
          class="group bg-white rounded-2xl p-6 border border-ink/[0.04] shadow-sm text-left hover:shadow-md hover:border-ink/[0.08] transition-all">
          <div class="flex items-center justify-between mb-5">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-ink/10 to-ink/[0.03] flex items-center justify-center">
              <Car class="w-6 h-6 text-ink/40" />
            </div>
            <ChevronRight class="w-5 h-5 text-ink/10 group-hover:text-ink/30 group-hover:translate-x-0.5 transition-all" />
          </div>
          <p class="font-display text-4xl text-ink tracking-wide">{stats.vehicles}</p>
          <p class="text-[14px] text-ink/35 mt-1 font-medium">Fleet Vehicles</p>
          <div class="mt-3 flex items-center gap-1.5">
            <Zap class="w-3.5 h-3.5 text-sage" />
            <span class="text-[13px] text-sage font-semibold">{evCount} electric</span>
          </div>
        </button>


        <button onclick={() => activeView = 'users'}
          class="group bg-white rounded-2xl p-6 border border-ink/[0.04] shadow-sm text-left hover:shadow-md hover:border-ink/[0.08] transition-all">
          <div class="flex items-center justify-between mb-5">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-crimson/10 to-crimson/[0.03] flex items-center justify-center">
              <Users class="w-6 h-6 text-crimson/50" />
            </div>
            <ChevronRight class="w-5 h-5 text-ink/10 group-hover:text-ink/30 group-hover:translate-x-0.5 transition-all" />
          </div>
          <p class="font-display text-4xl text-ink tracking-wide">{stats.users}</p>
          <p class="text-[14px] text-ink/35 mt-1 font-medium">Registered Users</p>
        </button>
      </div>


      <div class="flex flex-wrap gap-3 mb-8">
        <div class="flex items-center gap-2.5 px-5 py-2.5 bg-white rounded-xl border border-ink/[0.04] shadow-sm">
          <Activity class="w-4.5 h-4.5 text-sage" />
          <span class="text-[15px] font-bold text-ink">{activeCount}</span>
          <span class="text-[14px] text-ink/35">Active Rides</span>
        </div>
        <div class="flex items-center gap-2.5 px-5 py-2.5 bg-white rounded-xl border border-ink/[0.04] shadow-sm">
          <CheckCircle class="w-4.5 h-4.5 text-ink/25" />
          <span class="text-[15px] font-bold text-ink">{completedCount}</span>
          <span class="text-[14px] text-ink/35">Completed</span>
        </div>
        <div class="flex items-center gap-2.5 px-5 py-2.5 bg-white rounded-xl border border-ink/[0.04] shadow-sm">
          <Leaf class="w-4.5 h-4.5 text-sage" />
          <span class="text-[15px] font-bold text-ink">{totalCarbon.toFixed(1)} kg</span>
          <span class="text-[14px] text-ink/35">CO&#8322; Saved</span>
        </div>
      </div>


      <div class="grid grid-cols-1 lg:grid-cols-5 gap-6">


        <div class="lg:col-span-3 bg-white rounded-2xl border border-ink/[0.04] shadow-sm overflow-hidden">
          <div class="flex items-center justify-between px-6 py-5 border-b border-ink/[0.04]">
            <div class="flex items-center gap-3">
              <h2 class="text-[16px] font-semibold text-ink">Pending Approval</h2>
              {#if pending.length > 0}
                <span class="px-2 py-0.5 bg-crimson text-white text-[12px] font-bold rounded-full min-w-[24px] text-center">
                  {pending.length}
                </span>
              {/if}
            </div>
            <button onclick={() => activeView = 'bookings'}
              class="text-[13px] text-ink/30 hover:text-ink/50 transition-colors flex items-center gap-1 font-medium">
              All bookings <ChevronRight class="w-3.5 h-3.5" />
            </button>
          </div>
          {#if pending.length === 0}
            <div class="px-6 py-14 text-center">
              <div class="w-14 h-14 rounded-2xl bg-sage/8 flex items-center justify-center mx-auto mb-4">
                <CheckCircle class="w-7 h-7 text-sage/40" />
              </div>
              <p class="text-[15px] text-ink/35 font-medium">All caught up!</p>
              <p class="text-[13px] text-ink/20 mt-1">No bookings waiting for approval</p>
            </div>
          {:else}
            <div class="divide-y divide-ink/[0.04]">
              {#each pending.slice(0, 5) as b}
                <div class="px-6 py-4 flex items-center gap-4 hover:bg-paper/50 transition-colors">
                  <div class="w-10 h-10 rounded-xl bg-saffron/10 flex items-center justify-center shrink-0">
                    <Clock class="w-5 h-5 text-saffron" />
                  </div>
                  <div class="flex-1 min-w-0">
                    <p class="text-[15px] font-semibold text-ink truncate">
                      {b.vehicle?.make || ''} {b.vehicle?.model || 'Vehicle'}
                    </p>
                    <p class="text-[13px] text-ink/30 mt-0.5">
                      {b.renter?.name || b.renter?.phone || 'User'} &middot;
                      {fmt(b.start_time)} - {fmt(b.end_time)} &middot;
                      {payLbl[b.payment_method] || b.payment_method}
                    </p>
                  </div>
                  <p class="text-[15px] font-bold text-ink shrink-0 hidden sm:block">
                    NPR {(b.total_amount || 0).toLocaleString()}
                  </p>
                  <div class="flex items-center gap-2 shrink-0">
                    {#if actionLoading === b.id}
                      <Loader2 class="w-5 h-5 text-ink/20 animate-spin" />
                    {:else}
                      <button onclick={() => bookingAction(b.id, 'confirmed')} title="Approve"
                        class="w-9 h-9 rounded-xl bg-sage text-white flex items-center justify-center hover:bg-sage/90 active:scale-95 transition-all">
                        <CheckCircle class="w-4.5 h-4.5" />
                      </button>
                      <button onclick={() => bookingAction(b.id, 'cancelled')} title="Reject"
                        class="w-9 h-9 rounded-xl bg-ink/[0.04] text-ink/30 flex items-center justify-center hover:bg-crimson/10 hover:text-crimson transition-all">
                        <XCircle class="w-4.5 h-4.5" />
                      </button>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>


        <div class="lg:col-span-2 bg-white rounded-2xl border border-ink/[0.04] shadow-sm overflow-hidden">
          <div class="px-6 py-5 border-b border-ink/[0.04]">
            <h2 class="text-[16px] font-semibold text-ink">Recent Activity</h2>
          </div>
          {#if bookings.length === 0}
            <div class="px-6 py-14 text-center">
              <Clock class="w-8 h-8 text-ink/10 mx-auto mb-3" />
              <p class="text-[15px] text-ink/30">No activity yet</p>
            </div>
          {:else}
            <div class="divide-y divide-ink/[0.04]">
              {#each bookings.slice(0, 8) as b}
                <div class="px-6 py-3.5 flex items-center gap-3.5 hover:bg-paper/50 transition-colors">
                  <div class="w-2.5 h-2.5 rounded-full shrink-0 {stDot[b.status] || 'bg-ink/20'}"></div>
                  <div class="flex-1 min-w-0">
                    <p class="text-[14px] font-medium text-ink truncate">
                      {b.vehicle?.make || ''} {b.vehicle?.model || '-'}
                    </p>
                    <p class="text-[12px] text-ink/25 mt-0.5">
                      {fmt(b.start_time)} &middot; {b.renter?.name || b.renter?.phone || '-'}
                    </p>
                  </div>
                  <div class="text-right shrink-0">
                    <p class="text-[14px] font-bold text-ink">NPR {(b.total_amount || 0).toLocaleString()}</p>
                    <span class="text-[11px] font-semibold uppercase tracking-wider
                      {b.status === 'pending' ? 'text-saffron' : b.status === 'confirmed' || b.status === 'active' ? 'text-sage' : b.status === 'cancelled' ? 'text-crimson' : 'text-ink/30'}">
                      {b.status}
                    </span>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>
    </div>


  {:else if activeView === 'bookings'}
    <div class="max-w-7xl mx-auto px-6 lg:px-8 py-8">
      <div class="mb-6">
        <h1 class="font-display text-4xl text-ink tracking-wide">ALL BOOKINGS</h1>
        <p class="text-[15px] text-ink/35 mt-2">
          {bookings.length} bookings &middot; {pending.length} pending &middot; {activeCount} active
        </p>
      </div>

      {#if bookings.length === 0}
        <div class="bg-white rounded-2xl border border-ink/[0.04] shadow-sm text-center py-20">
          <div class="inline-flex items-center justify-center w-16 h-16 bg-ink/[0.03] rounded-2xl mb-4">
            <Calendar class="w-8 h-8 text-ink/15" />
          </div>
          <p class="text-[15px] text-ink/30 font-medium">No bookings yet</p>
        </div>
      {:else}
        <div class="bg-white rounded-2xl border border-ink/[0.04] shadow-sm overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full">
              <thead>
                <tr class="border-b border-ink/[0.06]">
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Vehicle</th>
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Renter</th>
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Period</th>
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Amount</th>
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Payment</th>
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Status</th>
                  <th class="text-right px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Actions</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-ink/[0.04]">
                {#each bookings as b}
                  <tr class="hover:bg-paper/50 transition-colors">
                    <td class="px-6 py-4">
                      <p class="text-[15px] font-semibold text-ink">{b.vehicle?.make || ''} {b.vehicle?.model || '-'}</p>
                      <p class="text-[12px] text-ink/25 capitalize mt-0.5">{b.vehicle?.type || ''}</p>
                    </td>
                    <td class="px-6 py-4">
                      <p class="text-[14px] text-ink/60 font-medium">{b.renter?.name || '-'}</p>
                      <p class="text-[12px] text-ink/25 font-mono mt-0.5">{b.renter?.phone || ''}</p>
                    </td>
                    <td class="px-6 py-4 text-[14px] text-ink/40">{fmt(b.start_time)} - {fmt(b.end_time)}</td>
                    <td class="px-6 py-4 text-[15px] font-bold text-ink">NPR {(b.total_amount || 0).toLocaleString()}</td>
                    <td class="px-6 py-4 text-[14px] text-ink/35">{payLbl[b.payment_method] || b.payment_method || '-'}</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex px-3 py-1 text-[12px] font-semibold tracking-wider uppercase rounded-full border {stBg[b.status] || stBg.pending}">
                        {b.status || 'pending'}
                      </span>
                    </td>
                    <td class="px-6 py-4 text-right">
                      {#if actionLoading === b.id}
                        <Loader2 class="w-4 h-4 text-ink/20 animate-spin inline" />
                      {:else if b.status === 'pending'}
                        <button onclick={() => bookingAction(b.id, 'confirmed')}
                          class="px-3.5 py-1.5 rounded-lg bg-sage text-white text-[12px] font-semibold uppercase tracking-wider hover:bg-sage/90 active:scale-95 transition-all mr-2">
                          Approve
                        </button>
                        <button onclick={() => bookingAction(b.id, 'cancelled')}
                          class="px-3.5 py-1.5 rounded-lg bg-ink/[0.04] text-ink/40 text-[12px] font-semibold uppercase tracking-wider hover:bg-crimson/10 hover:text-crimson transition-all">
                          Reject
                        </button>
                      {:else if b.status === 'confirmed'}
                        <button onclick={() => bookingAction(b.id, 'active')}
                          class="px-3.5 py-1.5 rounded-lg bg-sage/10 text-sage text-[12px] font-semibold uppercase tracking-wider hover:bg-sage/20 active:scale-95 transition-all mr-2">
                          Activate
                        </button>
                        <button onclick={() => bookingAction(b.id, 'cancelled')}
                          class="px-3.5 py-1.5 rounded-lg bg-ink/[0.04] text-ink/40 text-[12px] font-semibold uppercase tracking-wider hover:bg-crimson/10 hover:text-crimson transition-all">
                          Cancel
                        </button>
                      {:else if b.status === 'active'}
                        <button onclick={() => bookingAction(b.id, 'completed')}
                          class="px-3.5 py-1.5 rounded-lg bg-ink/[0.06] text-ink/50 text-[12px] font-semibold uppercase tracking-wider hover:bg-ink/10 active:scale-95 transition-all">
                          Complete
                        </button>
                      {:else}
                        <span class="text-ink/10 text-[14px]">&mdash;</span>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      {/if}
    </div>


  {:else if activeView === 'vehicles'}
    <div class="max-w-7xl mx-auto px-6 lg:px-8 py-8">
      <div class="flex items-center justify-between mb-6">
        <div>
          <h1 class="font-display text-4xl text-ink tracking-wide">FLEET MANAGEMENT</h1>
          <p class="text-[15px] text-ink/35 mt-2">{vehicles.length} vehicles &middot; {evCount} electric</p>
        </div>
        <button onclick={() => showAddVehicle = true}
          class="flex items-center gap-2 px-5 py-2.5 bg-ink text-paper text-[14px] font-semibold rounded-xl hover:bg-ink/90 active:scale-[0.98] transition-all">
          <Plus class="w-4 h-4" /> Add Vehicle
        </button>
      </div>

      {#if vehicles.length === 0}
        <div class="bg-white rounded-2xl border border-ink/[0.04] shadow-sm text-center py-20">
          <div class="inline-flex items-center justify-center w-16 h-16 bg-ink/[0.03] rounded-2xl mb-4">
            <Car class="w-8 h-8 text-ink/15" />
          </div>
          <p class="text-[15px] text-ink/30 font-medium mb-4">No vehicles listed</p>
          <button onclick={() => showAddVehicle = true}
            class="px-5 py-2.5 bg-ink text-paper text-[14px] font-semibold rounded-xl hover:bg-ink/90 transition-colors">
            Add First Vehicle
          </button>
        </div>
      {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5">
          {#each vehicles as v}
            <div class="bg-white rounded-2xl border border-ink/[0.04] shadow-sm overflow-hidden hover:shadow-md hover:border-ink/[0.08] transition-all group">

              <div class="relative h-44 bg-ink/[0.02]">
                <img
                  src={v.image_url || 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=400&h=200&fit=crop'}
                  alt="{v.make} {v.model}"
                  class="w-full h-full object-cover"
                  onerror={(e) => { (e.target as HTMLImageElement).src = 'https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=400&h=200&fit=crop'; }}
                />
                <div class="absolute top-3 left-3 flex gap-2">
                  {#if v.is_ev}
                    <span class="px-2.5 py-1 bg-sage text-white text-[12px] font-semibold rounded-full flex items-center gap-1 shadow-lg">
                      <Zap class="w-3 h-3" /> EV
                    </span>
                  {/if}
                  <span class="px-2.5 py-1 bg-white/90 backdrop-blur-sm text-ink text-[12px] font-semibold rounded-full capitalize shadow-lg">
                    {v.type}
                  </span>
                </div>
                <div class="absolute top-3 right-3">
                  <span class="px-2.5 py-1 text-[11px] font-bold uppercase rounded-full shadow-lg
                    {v.isAvailable || v.available ? 'bg-sage text-white' : 'bg-crimson text-white'}">
                    {v.isAvailable || v.available ? 'Live' : 'Off'}
                  </span>
                </div>
              </div>


              <div class="p-5">
                <h3 class="text-[16px] font-semibold text-ink">{v.make} {v.model}</h3>
                <p class="text-[13px] text-ink/30 mt-1 flex items-center gap-1.5">
                  <MapPin class="w-3.5 h-3.5" />
                  {v.location || v.location_name || '-'}
                  {#if v.plate_no}
                    &middot; <span class="font-mono">{v.plate_no}</span>
                  {/if}
                </p>
                {#if v.owner_name}
                  <p class="text-[13px] text-ink/25 mt-0.5">Owner: {v.owner_name}</p>
                {/if}

                <div class="flex items-center justify-between mt-4 pt-4 border-t border-ink/[0.04]">
                  <div>
                    <span class="font-display text-xl text-ink">NPR {(v.daily_rate || 0).toLocaleString()}</span>
                    <span class="text-[12px] text-ink/25">/day</span>
                  </div>
                  <div class="flex items-center gap-2">
                    <a href="/vehicles/{v.id}"
                      class="w-9 h-9 rounded-xl bg-ink/[0.04] text-ink/30 flex items-center justify-center hover:bg-ink/[0.08] hover:text-ink/50 transition-all"
                      title="View">
                      <Eye class="w-4 h-4" />
                    </a>
                    {#if deleteConfirm === v.id}
                      <button onclick={() => removeVehicle(v.id)}
                        class="px-3 py-1.5 rounded-xl bg-crimson text-white text-[12px] font-semibold hover:bg-crimson/90 active:scale-95 transition-all">
                        {actionLoading === v.id ? '...' : 'Confirm'}
                      </button>
                      <button onclick={() => deleteConfirm = ''}
                        class="w-9 h-9 rounded-xl bg-ink/[0.04] text-ink/30 flex items-center justify-center hover:bg-ink/[0.08] transition-all">
                        <X class="w-4 h-4" />
                      </button>
                    {:else}
                      <button onclick={() => deleteConfirm = v.id}
                        class="w-9 h-9 rounded-xl bg-crimson/5 text-crimson/40 flex items-center justify-center hover:bg-crimson/10 hover:text-crimson transition-all"
                        title="Delete">
                        <Trash2 class="w-4 h-4" />
                      </button>
                    {/if}
                  </div>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>


  {:else if activeView === 'users'}
    <div class="max-w-7xl mx-auto px-6 lg:px-8 py-8">
      <div class="mb-6">
        <h1 class="font-display text-4xl text-ink tracking-wide">USERS</h1>
        <p class="text-[15px] text-ink/35 mt-2">{users_list.length} registered users</p>
      </div>

      {#if users_list.length === 0}
        <div class="bg-white rounded-2xl border border-ink/[0.04] shadow-sm text-center py-20">
          <div class="inline-flex items-center justify-center w-16 h-16 bg-ink/[0.03] rounded-2xl mb-4">
            <Users class="w-8 h-8 text-ink/15" />
          </div>
          <p class="text-[15px] text-ink/30 font-medium">No users found</p>
        </div>
      {:else}
        <div class="bg-white rounded-2xl border border-ink/[0.04] shadow-sm overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full">
              <thead>
                <tr class="border-b border-ink/[0.06]">
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">User</th>
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Phone</th>
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Role</th>
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Language</th>
                  <th class="text-left px-6 py-4 text-[12px] font-semibold tracking-wider uppercase text-ink/25">Joined</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-ink/[0.04]">
                {#each users_list as u}
                  <tr class="hover:bg-paper/50 transition-colors">
                    <td class="px-6 py-4">
                      <div class="flex items-center gap-3">
                        <div class="w-10 h-10 rounded-xl flex items-center justify-center text-[14px] font-bold
                          {roleBg[u.role] || roleBg.consumer}">
                          {(u.name || u.phone || '?').charAt(0).toUpperCase()}
                        </div>
                        <span class="text-[15px] font-semibold text-ink">{u.name || '-'}</span>
                      </div>
                    </td>
                    <td class="px-6 py-4 text-[14px] text-ink/40 font-mono">{u.phone || '-'}</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex px-3 py-1 text-[12px] font-semibold tracking-wider uppercase rounded-full capitalize
                        {roleBg[u.role] || roleBg.consumer}">
                        {u.role || 'consumer'}
                      </span>
                    </td>
                    <td class="px-6 py-4 text-[14px] text-ink/35">
                      {u.lang_pref === 'ne' ? '🇳🇵 Nepali' : '🇬🇧 English'}
                    </td>
                    <td class="px-6 py-4 text-[14px] text-ink/35">{u.created_at ? fmtFull(u.created_at) : '-'}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>


{#if showAddVehicle}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-50 flex items-start justify-center p-4 pt-20 overflow-y-auto"
    onclick={() => showAddVehicle = false}>
    <div class="absolute inset-0 bg-ink/60 backdrop-blur-sm"></div>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="relative bg-white rounded-2xl w-full max-w-lg shadow-2xl"
      onclick={(e) => e.stopPropagation()}>


      <div class="flex items-center justify-between px-6 py-5 border-b border-ink/[0.04]">
        <h2 class="text-[16px] font-semibold text-ink">Add New Vehicle</h2>
        <button onclick={() => showAddVehicle = false}
          class="p-2 hover:bg-paper rounded-xl transition-colors">
          <X class="w-5 h-5 text-ink/30" />
        </button>
      </div>


      <div class="p-6 space-y-5 max-h-[70vh] overflow-y-auto">

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-[13px] font-semibold text-ink/40 mb-2">Type</label>
            <select bind:value={newVehicle.type}
              class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10">
              <option value="car">Car</option>
              <option value="suv">SUV</option>
              <option value="bike">Bike</option>
              <option value="scooter">Scooter</option>
              <option value="jeep">Jeep</option>
            </select>
          </div>
          <div>
            <label class="block text-[13px] font-semibold text-ink/40 mb-2">Electric Vehicle?</label>
            <div class="flex gap-2 h-[46px]">
              <button type="button" onclick={() => newVehicle.is_ev = true}
                class="flex-1 rounded-xl text-[14px] font-semibold border-2 transition-all
                  {newVehicle.is_ev ? 'border-sage bg-sage/10 text-sage' : 'border-ink/[0.04] text-ink/30 hover:border-ink/10'}">
                Yes
              </button>
              <button type="button" onclick={() => newVehicle.is_ev = false}
                class="flex-1 rounded-xl text-[14px] font-semibold border-2 transition-all
                  {!newVehicle.is_ev ? 'border-ink bg-ink/5 text-ink' : 'border-ink/[0.04] text-ink/30 hover:border-ink/10'}">
                No
              </button>
            </div>
          </div>
        </div>


        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-[13px] font-semibold text-ink/40 mb-2">Make</label>
            <input bind:value={newVehicle.make} placeholder="e.g. Tata"
              class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 placeholder:text-ink/20" />
          </div>
          <div>
            <label class="block text-[13px] font-semibold text-ink/40 mb-2">Model</label>
            <input bind:value={newVehicle.model} placeholder="e.g. Nexon EV"
              class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 placeholder:text-ink/20" />
          </div>
        </div>


        <div class="grid grid-cols-3 gap-4">
          <div>
            <label class="block text-[13px] font-semibold text-ink/40 mb-2">Year</label>
            <input type="number" bind:value={newVehicle.year}
              class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10" />
          </div>
          <div class="col-span-2">
            <label class="block text-[13px] font-semibold text-ink/40 mb-2">Plate Number</label>
            <input bind:value={newVehicle.plate_no} placeholder="Ba 1 Pa 1234"
              class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 placeholder:text-ink/20" />
          </div>
        </div>


        {#if newVehicle.is_ev}
          <div>
            <label class="block text-[13px] font-semibold text-ink/40 mb-2">EV Range (km)</label>
            <input type="number" bind:value={newVehicle.ev_range_km}
              class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10" />
          </div>
        {/if}


        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-[13px] font-semibold text-ink/40 mb-2">Hourly Rate (NPR)</label>
            <input type="number" bind:value={newVehicle.hourly_rate}
              class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10" />
          </div>
          <div>
            <label class="block text-[13px] font-semibold text-ink/40 mb-2">Daily Rate (NPR)</label>
            <input type="number" bind:value={newVehicle.daily_rate}
              class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10" />
          </div>
        </div>


        <div>
          <label class="block text-[13px] font-semibold text-ink/40 mb-2">Location</label>
          <select bind:value={newVehicle.location_name}
            class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10">
            <option value="Kathmandu">Kathmandu</option>
            <option value="Thamel">Thamel</option>
            <option value="Patan">Patan</option>
            <option value="Bhaktapur">Bhaktapur</option>
            <option value="Lalitpur">Lalitpur</option>
            <option value="Bouddha">Bouddha</option>
          </select>
        </div>


        <div>
          <label class="block text-[13px] font-semibold text-ink/40 mb-2">Description</label>
          <textarea bind:value={newVehicle.description} rows="2" placeholder="Brief description of the vehicle..."
            class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 resize-none placeholder:text-ink/20">
          </textarea>
        </div>


        <div>
          <label class="block text-[13px] font-semibold text-ink/40 mb-2">Image URL</label>
          <input bind:value={newVehicle.image_url} placeholder="https://..."
            class="w-full px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 placeholder:text-ink/20" />
        </div>


        <div>
          <label class="block text-[13px] font-semibold text-ink/40 mb-2">Features</label>
          <div class="flex gap-2 mb-3">
            <input bind:value={newFeature} placeholder="AC, GPS, Bluetooth..."
              class="flex-1 px-4 py-3 bg-paper border border-ink/[0.06] rounded-xl text-[15px] focus:outline-none focus:border-sage focus:ring-2 focus:ring-sage/10 placeholder:text-ink/20"
              onkeydown={(e) => { if (e.key === 'Enter') { e.preventDefault(); addFeat(); } }} />
            <button type="button" onclick={addFeat}
              class="px-4 py-3 bg-ink/[0.04] text-ink/50 text-[14px] font-semibold rounded-xl hover:bg-ink/[0.08] transition-colors">
              Add
            </button>
          </div>
          {#if newVehicle.features.length > 0}
            <div class="flex flex-wrap gap-2">
              {#each newVehicle.features as f}
                <span class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-paper text-ink/60 text-[13px] font-medium rounded-lg border border-ink/[0.04]">
                  {f}
                  <button onclick={() => remFeat(f)} class="text-ink/25 hover:text-crimson transition-colors">
                    <X class="w-3.5 h-3.5" />
                  </button>
                </span>
              {/each}
            </div>
          {/if}
        </div>

        {#if actionError}
          <div class="p-4 bg-crimson/5 border border-crimson/15 rounded-xl text-[14px] text-crimson font-medium">{actionError}</div>
        {/if}
      </div>


      <div class="px-6 py-5 border-t border-ink/[0.04] flex items-center justify-end gap-3">
        <button onclick={() => showAddVehicle = false}
          class="px-5 py-2.5 text-[14px] text-ink/40 hover:text-ink/60 font-medium transition-colors">
          Cancel
        </button>
        <button
          onclick={addVehicle}
          disabled={addingVehicle || !newVehicle.make || !newVehicle.model || !newVehicle.plate_no}
          class="px-6 py-2.5 bg-ink text-paper text-[14px] font-semibold rounded-xl hover:bg-ink/90 active:scale-[0.98] disabled:opacity-40 disabled:cursor-not-allowed transition-all"
        >
          {#if addingVehicle}
            <span class="inline-flex items-center gap-2"><Loader2 class="w-4 h-4 animate-spin" /> Adding...</span>
          {:else}
            Add Vehicle
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
