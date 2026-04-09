<script lang="ts">
  import { Mail, Phone, MapPin } from 'lucide-svelte';
</script>

<footer class="bg-ink text-paper/40">
  <div class="max-w-7xl mx-auto px-6 lg:px-8 py-16">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-10">
      <div>
        <div class="flex items-center gap-1.5 mb-4">
          <span class="font-display text-2xl tracking-wide text-paper">YATRA</span>
          <span class="font-display text-2xl tracking-wide text-saffron">SATHI</span>
        </div>
        <p class="text-[14px] text-paper/25 leading-relaxed">
          Nepal's premier EV rental marketplace. Explore Kathmandu Valley sustainably.
        </p>
      </div>

      <div>
        <h3 class="text-[13px] font-semibold tracking-wider uppercase text-paper/30 mb-4">Quick Links</h3>
        <ul class="space-y-2.5">
          <li><a href="/" class="text-[14px] text-paper/25 hover:text-saffron transition-colors">Home</a></li>
          <li><a href="/#vehicles" class="text-[14px] text-paper/25 hover:text-saffron transition-colors">Vehicles</a></li>
          <li><a href="/bookings" class="text-[14px] text-paper/25 hover:text-saffron transition-colors">Bookings</a></li>
          <li><a href="/auth" class="text-[14px] text-paper/25 hover:text-saffron transition-colors">Login</a></li>
        </ul>
      </div>

      <div>
        <h3 class="text-[13px] font-semibold tracking-wider uppercase text-paper/30 mb-4">Contact</h3>
        <ul class="space-y-2.5">
          <li class="flex items-center gap-2.5 text-[14px] text-paper/25"><MapPin class="w-4 h-4 text-sage shrink-0" /> Thamel, Kathmandu</li>
          <li class="flex items-center gap-2.5 text-[14px] text-paper/25"><Phone class="w-4 h-4 text-sage shrink-0" /> +977 9800-000000</li>
          <li class="flex items-center gap-2.5 text-[14px] text-paper/25"><Mail class="w-4 h-4 text-sage shrink-0" /> info@yatrasathi.org</li>
        </ul>
      </div>

      <div>
        <h3 class="text-[13px] font-semibold tracking-wider uppercase text-paper/30 mb-4">Newsletter</h3>
        <p class="text-[14px] text-paper/25 mb-4">Stay updated with latest EV offerings.</p>
        <form onsubmit={(e) => e.preventDefault()} class="flex gap-2">
          <input type="email" placeholder="Your email"
            class="flex-1 px-4 py-2.5 bg-paper/5 border border-paper/10 rounded-xl text-[14px] text-paper placeholder:text-paper/15 focus:outline-none focus:border-saffron/40 transition-colors" />
          <button type="submit"
            class="px-4 py-2.5 bg-saffron text-ink text-[14px] font-semibold rounded-xl hover:bg-saffron-300 transition-colors">
            Join
          </button>
        </form>
      </div>
    </div>
  </div>

  <div class="border-t border-paper/5">
    <div class="max-w-7xl mx-auto px-6 lg:px-8 py-5 flex flex-col sm:flex-row items-center justify-between gap-3">
      <p class="text-[13px] text-paper/15">&copy; 2026 YatraSathi. Empowering Nepal's green mobility.</p>
      <div class="flex items-center gap-5">
        <a href="/" class="text-paper/15 hover:text-paper/30 transition-colors text-[13px]">Privacy</a>
        <a href="/" class="text-paper/15 hover:text-paper/30 transition-colors text-[13px]">Terms</a>
        <a href="/" class="text-paper/15 hover:text-paper/30 transition-colors text-[13px]">Support</a>
      </div>
    </div>
  </div>
</footer>
