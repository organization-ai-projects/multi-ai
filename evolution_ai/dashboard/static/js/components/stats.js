class StatsGrid {
  constructor() {
    this.container = document.getElementById('stats-grid');
    this.init();
  }

  init() {
    this.container.className = 'stats-grid';
    this.container.innerHTML = `
            <div class="stat-card">
                <div id="currentGen" class="stat-value">0</div>
                <div class="stat-label">Génération</div>
            </div>
            <div class="stat-card">
                <div id="diversity" class="stat-value">0</div>
                <div class="stat-label">Diversité</div>
            </div>
            <div class="stat-card">
                <div id="bestScore" class="stat-value">0</div>
                <div class="stat-label">Meilleur Score</div>
            </div>
            <div class="stat-card">
                <div id="totalSpecies" class="stat-value">0</div>
                <div class="stat-label">Espèces Actives</div>
            </div>
        `;
  }

  update(data) {
    document.getElementById('currentGen').textContent = data.current_generation;
    document.getElementById('diversity').textContent = data.global_diversity.toFixed(3);
    document.getElementById('bestScore').textContent = Math.max(
      ...data.best_performers.map((p) => p[1])
    ).toFixed(2);
    document.getElementById('totalSpecies').textContent = Object.keys(data.species_stats).length;
  }
}
