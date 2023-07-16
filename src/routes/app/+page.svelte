<script lang="ts">
</script>

<div class="drawer md:drawer-open">
  <input id="sidebar" type="checkbox" class="drawer-toggle" />
  <div class="drawer-content flex flex-col items-center justify-center">
    <!-- Page content here -->
    <label for="sidebar" class="btn btn-primary drawer-button md:hidden">Open drawer</label>
  </div>
  <div class="drawer-side">
    <label for="sidebar" class="drawer-overlay" />
    <ul class="menu p-4 w-60 h-full bg-base-200 text-base-content">
      <!-- Sidebar content here -->
      <li>Sidebar Item 1</li>
      <li>Sidebar Item 2</li>
    </ul>
  </div>
</div>
