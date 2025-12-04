<div class="topbar" data-tauri-drag-region>
  <span class="topbar-title">My App</span>
</div>
<style>
.topbar {
  height: 38px;
  width: 100%;
  display: flex;
  justify-content: center;   /* center the title */
  align-items: center;
  -webkit-app-region: drag;  /* make the bar draggable */
  background: rgba(32, 32, 32, 0.397);   /* no borders, no bg */
}

.topbar-title {
  user-select: none;
  pointer-events: none;      /* prevent blocking drag */
  font-size: 14px;
  font-weight: 500;
  opacity: 0.7;
}

</style>