<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAdmin } from '$lib/stores/auth';
  import { getAdminStats, getAdminUsers, getAdminBookings, getAdminVehicles } from '$lib/api';
  import {
    Users, Car, Calendar, DollarSign, Loader2, Shield
  } from 'lucide-svelte';

  let stats = $state({ users: 0, vehicles: 0, bookings: 0, revenue: 0 });
  let users = $state<any[]>([]);
  let vehicles = $state<any[]>([]);
  let bookings = $state<any[]>([]);
  let loading = $state(true);
  let activeTab = $state<'users' | 'vehicles' | 'bookings'>('users');

  onMount(() => {
    const unsubscribe = isAdmin.subscribe((val) => {
      if (!val) goto('/');
    });

    loadData();
    return unsubscribe;
  });

  async function loadData() {
    loading = true;
    try {
      const [statsRes, usersRes, vehiclesRes, bookingsRes] = await Promise.all([
        getAdminStats().catch(() => ({ users: 0, vehicles: 0, bookings: 0, revenue: 0 })),
        getAdminUsers().catch(() => ({ users: [] })),
        getAdminVehicles().catch(() => ({ vehicles: [] })),
        getAdminBookings().catch(() => ({ bookings: [] })),
      ]);

      stats = {
        users: statsRes.users || statsRes.totalUsers || 0,
        vehicles: statsRes.vehicles || statsRes.totalVehicles || 0,
        bookings: statsRes.bookings || statsRes.totalBookings || 0,
        revenue: statsRes.revenue || statsRes.totalRevenue || 0,
      };
      users = usersRes.users || usersRes || [];
      vehicles = vehiclesRes.vehicles || vehiclesRes || [];
      bookings = bookingsRes.bookings || bookingsRes || [];
    } catch {
      // Keep defaults
    } finally {
      loading = false;
    }
  }

  function formatDate(dateStr: string) {
    try {
      return new Date(dateStr).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
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

  const roleColors: Record<string, string> = {
    user: 'bg-slate-100 text-slate-700',
    owner: 'bg-emerald-100 text-emerald-700',
    driver: 'bg-blue-100 text-blue-700',
    admin: 'bg-purple-100 text-purple-700',
  };
</script>

<svelte:head>
  <title>Admin Panel - YatraSathi</title>
</svelte:head>

<div class="pt-20 pb-16 bg-slate-50 min-h-screen">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <!-- Header -->
    <div class="flex items-center gap-3 mb-8">
      <div class="w-10 h-10 bg-purple-100 rounded-xl flex items-center justify-center">
        <Shield class="w-5 h-5 text-purple-600" />
      </div>
      <div>
        <h1 class="text-3xl font-bold text-slate-900">Admin Panel</h1>
        <p class="text-slate-500">Manage users, vehicles, and bookings</p>
      </div>
    </div>

    {#if loading}
      <div class="flex items-center justify-center py-20">
        <Loader2 class="w-8 h-8 text-emerald-600 animate-spin" />
      </div>
    {:else}
      <!-- Stats Cards -->
      <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <div class="bg-white rounded-2xl p-5 shadow-sm border border-slate-100">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-blue-100 rounded-xl flex items-center justify-center">
              <Users class="w-5 h-5 text-blue-600" />
            </div>
            <div>
              <p class="text-xs text-slate-500 uppercase tracking-wide">Users</p>
              <p class="text-xl font-bold text-slate-900">{stats.users}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-2xl p-5 shadow-sm border border-slate-100">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-emerald-100 rounded-xl flex items-center justify-center">
              <Car class="w-5 h-5 text-emerald-600" />
            </div>
            <div>
              <p class="text-xs text-slate-500 uppercase tracking-wide">Vehicles</p>
              <p class="text-xl font-bold text-slate-900">{stats.vehicles}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-2xl p-5 shadow-sm border border-slate-100">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-amber-100 rounded-xl flex items-center justify-center">
              <Calendar class="w-5 h-5 text-amber-600" />
            </div>
            <div>
              <p class="text-xs text-slate-500 uppercase tracking-wide">Bookings</p>
              <p class="text-xl font-bold text-slate-900">{stats.bookings}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-2xl p-5 shadow-sm border border-slate-100">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-purple-100 rounded-xl flex items-center justify-center">
              <DollarSign class="w-5 h-5 text-purple-600" />
            </div>
            <div>
              <p class="text-xs text-slate-500 uppercase tracking-wide">Revenue</p>
              <p class="text-xl font-bold text-slate-900">NPR {stats.revenue.toLocaleString()}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Tabs -->
      <div class="bg-white rounded-2xl shadow-sm border border-slate-100 overflow-hidden">
        <div class="flex border-b border-slate-100">
          <button
            onclick={() => activeTab = 'users'}
            class="px-6 py-3.5 text-sm font-medium transition-colors border-b-2 {activeTab === 'users' ? 'border-emerald-600 text-emerald-700 bg-emerald-50/50' : 'border-transparent text-slate-500 hover:text-slate-700'}"
          >
            Users ({users.length})
          </button>
          <button
            onclick={() => activeTab = 'vehicles'}
            class="px-6 py-3.5 text-sm font-medium transition-colors border-b-2 {activeTab === 'vehicles' ? 'border-emerald-600 text-emerald-700 bg-emerald-50/50' : 'border-transparent text-slate-500 hover:text-slate-700'}"
          >
            Vehicles ({vehicles.length})
          </button>
          <button
            onclick={() => activeTab = 'bookings'}
            class="px-6 py-3.5 text-sm font-medium transition-colors border-b-2 {activeTab === 'bookings' ? 'border-emerald-600 text-emerald-700 bg-emerald-50/50' : 'border-transparent text-slate-500 hover:text-slate-700'}"
          >
            Bookings ({bookings.length})
          </button>
        </div>

        <div class="overflow-x-auto">
          {#if activeTab === 'users'}
            {#if users.length === 0}
              <div class="text-center py-16">
                <Users class="w-10 h-10 text-slate-300 mx-auto mb-3" />
                <p class="text-sm text-slate-500">No users found</p>
              </div>
            {:else}
              <table class="w-full">
                <thead>
                  <tr class="bg-slate-50">
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Phone</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Name</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Role</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Joined</th>
                  </tr>
                </thead>
                <tbody>
                  {#each users as user, i}
                    <tr class="{i % 2 === 0 ? 'bg-white' : 'bg-slate-50/50'} hover:bg-emerald-50/30 transition-colors">
                      <td class="px-6 py-3.5 text-sm text-slate-900 font-medium">{user.phone || '-'}</td>
                      <td class="px-6 py-3.5 text-sm text-slate-600">{user.name || '-'}</td>
                      <td class="px-6 py-3.5">
                        <span class="px-2.5 py-1 text-xs font-semibold rounded-lg capitalize {roleColors[user.role] || roleColors.user}">
                          {user.role || 'user'}
                        </span>
                      </td>
                      <td class="px-6 py-3.5 text-sm text-slate-500">{user.createdAt ? formatDate(user.createdAt) : '-'}</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            {/if}
          {:else if activeTab === 'vehicles'}
            {#if vehicles.length === 0}
              <div class="text-center py-16">
                <Car class="w-10 h-10 text-slate-300 mx-auto mb-3" />
                <p class="text-sm text-slate-500">No vehicles found</p>
              </div>
            {:else}
              <table class="w-full">
                <thead>
                  <tr class="bg-slate-50">
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Vehicle</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Type</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Location</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Price/Day</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {#each vehicles as vehicle, i}
                    <tr class="{i % 2 === 0 ? 'bg-white' : 'bg-slate-50/50'} hover:bg-emerald-50/30 transition-colors">
                      <td class="px-6 py-3.5 text-sm text-slate-900 font-medium">{vehicle.make} {vehicle.model} ({vehicle.year})</td>
                      <td class="px-6 py-3.5 text-sm text-slate-600 capitalize">{vehicle.type}</td>
                      <td class="px-6 py-3.5 text-sm text-slate-600">{vehicle.location || '-'}</td>
                      <td class="px-6 py-3.5 text-sm text-slate-900 font-medium">NPR {vehicle.pricePerDay?.toLocaleString()}</td>
                      <td class="px-6 py-3.5">
                        <span class="px-2.5 py-1 text-xs font-semibold rounded-lg {vehicle.isAvailable ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'}">
                          {vehicle.isAvailable ? 'Available' : 'Unavailable'}
                        </span>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            {/if}
          {:else}
            {#if bookings.length === 0}
              <div class="text-center py-16">
                <Calendar class="w-10 h-10 text-slate-300 mx-auto mb-3" />
                <p class="text-sm text-slate-500">No bookings found</p>
              </div>
            {:else}
              <table class="w-full">
                <thead>
                  <tr class="bg-slate-50">
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Vehicle</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Dates</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Amount</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Payment</th>
                    <th class="text-left px-6 py-3 text-xs font-semibold text-slate-500 uppercase tracking-wide">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {#each bookings as booking, i}
                    <tr class="{i % 2 === 0 ? 'bg-white' : 'bg-slate-50/50'} hover:bg-emerald-50/30 transition-colors">
                      <td class="px-6 py-3.5 text-sm text-slate-900 font-medium">
                        {#if booking.vehicle}
                          {booking.vehicle.make} {booking.vehicle.model}
                        {:else}
                          -
                        {/if}
                      </td>
                      <td class="px-6 py-3.5 text-sm text-slate-600">
                        {formatDate(booking.startDate)} - {formatDate(booking.endDate)}
                      </td>
                      <td class="px-6 py-3.5 text-sm text-slate-900 font-medium">NPR {(booking.totalAmount || 0).toLocaleString()}</td>
                      <td class="px-6 py-3.5 text-sm text-slate-600 capitalize">{booking.paymentMethod || '-'}</td>
                      <td class="px-6 py-3.5">
                        <span class="px-2.5 py-1 text-xs font-semibold rounded-lg capitalize {statusColors[booking.status] || statusColors.pending}">
                          {booking.status || 'pending'}
                        </span>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            {/if}
          {/if}
        </div>
      </div>
    {/if}
  </div>
</div>
