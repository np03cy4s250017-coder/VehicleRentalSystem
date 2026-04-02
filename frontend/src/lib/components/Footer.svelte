<script lang="ts">
  import { Zap, Mail, Phone, MapPin } from 'lucide-svelte';
</script>

<footer class="bg-slate-900 text-slate-300">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-10">
      <!-- About -->
      <div>
        <div class="flex items-center gap-2 mb-4">
          <span class="text-2xl">🏔️</span>
          <span class="text-xl font-bold text-white">Yatra<span class="text-amber-500">Sathi</span></span>
          <Zap class="w-4 h-4 text-sky-400" />
        </div>
        <p class="text-sm text-slate-400 leading-relaxed">
          Nepal's premier electric vehicle rental marketplace. Explore the beauty of Kathmandu Valley sustainably with our fleet of eco-friendly EVs.
        </p>
      </div>

      <!-- Quick Links -->
      <div>
        <h3 class="text-white font-semibold text-sm uppercase tracking-wider mb-4">Quick Links</h3>
        <ul class="space-y-3">
          <li><a href="/" class="text-sm text-slate-400 hover:text-emerald-400 transition-colors">Home</a></li>
          <li><a href="/#vehicles" class="text-sm text-slate-400 hover:text-emerald-400 transition-colors">Browse Vehicles</a></li>
          <li><a href="/bookings" class="text-sm text-slate-400 hover:text-emerald-400 transition-colors">My Bookings</a></li>
          <li><a href="/auth" class="text-sm text-slate-400 hover:text-emerald-400 transition-colors">Login / Register</a></li>
        </ul>
      </div>

      <!-- Contact -->
      <div>
        <h3 class="text-white font-semibold text-sm uppercase tracking-wider mb-4">Contact Us</h3>
        <ul class="space-y-3">
          <li class="flex items-center gap-3 text-sm text-slate-400">
            <MapPin class="w-4 h-4 text-emerald-500 shrink-0" />
            Thamel, Kathmandu, Nepal
          </li>
          <li class="flex items-center gap-3 text-sm text-slate-400">
            <Phone class="w-4 h-4 text-emerald-500 shrink-0" />
            +977 9800-000000
          </li>
          <li class="flex items-center gap-3 text-sm text-slate-400">
            <Mail class="w-4 h-4 text-emerald-500 shrink-0" />
            info@yatrasathi.com
          </li>
        </ul>
      </div>

      <!-- Newsletter -->
      <div>
        <h3 class="text-white font-semibold text-sm uppercase tracking-wider mb-4">Newsletter</h3>
        <p class="text-sm text-slate-400 mb-4">Stay updated with our latest EV offerings and deals.</p>
        <form onsubmit={(e) => e.preventDefault()} class="flex gap-2">
          <input
            type="email"
            placeholder="Your email"
            class="flex-1 px-4 py-2 bg-slate-800 border border-slate-700 rounded-lg text-sm text-white placeholder:text-slate-500 focus:outline-none focus:border-emerald-500 transition-colors"
          />
          <button
            type="submit"
            class="px-4 py-2 bg-emerald-600 text-white text-sm font-medium rounded-lg hover:bg-emerald-700 transition-colors"
          >
            Join
          </button>
        </form>
      </div>
    </div>
  </div>

  <!-- Bottom Bar -->
  <div class="border-t border-slate-800">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6 flex flex-col sm:flex-row items-center justify-between gap-4">
      <p class="text-sm text-slate-500">
        &copy; 2024 YatraSathi. Empowering Nepal's green mobility.
      </p>
      <div class="flex items-center gap-4">
        <a href="/" class="text-slate-500 hover:text-emerald-400 transition-colors text-sm">Privacy</a>
        <a href="/" class="text-slate-500 hover:text-emerald-400 transition-colors text-sm">Terms</a>
        <a href="/" class="text-slate-500 hover:text-emerald-400 transition-colors text-sm">Support</a>
      </div>
    </div>
  </div>
</footer>
