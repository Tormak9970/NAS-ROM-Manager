<div class="hero-placeholder">
  <div class="pattern"></div>
</div>

<style>
  .hero-placeholder {
    width: 100%;
    height: 100%;
    
    /* background-color: rgb(var(--m3-scheme-surface-container-high)); */
  }
  .pattern {
    width: 100%;
    height: 100%;
    mask-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='200' height='200'><text x='15' y='150' transform='rotate(-45 0 150)' font-size='20' fill='white' font-family='Arial'>NAS ROM Manager</text></svg>");
    mask-repeat: repeat;
    mask-size: 200px;
    
    background-color: rgb(var(--m3-scheme-secondary-container));
  }
</style>