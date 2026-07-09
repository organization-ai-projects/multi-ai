class App {
  constructor() {
    this.api = new API();
    this.statsGrid = new StatsGrid();
    this.controlPanel = new ControlPanel(this.api);
    this.logPanel = new LogPanel();

    // Rendre le logPanel accessible globalement pour les notifications
    window.logPanel = this.logPanel;

    this.charts = {
      species: null,
      performance: null,
      diversity: null,
      timeline: null,
    };

    this.data = {
      diversityHistory: [],
      performanceHistory: [],
      speciesHistory: new Map(),
    };

    this.initCharts();
    this.startPolling();
  }

  initCharts() {
    this.charts.species = new SpeciesChart('speciesChart');
    this.charts.performance = new PerformanceChart('performanceChart');
    this.charts.diversity = new DiversityChart('diversityChart');
    this.charts.timeline = new TimelineChart('speciesTimelineChart');
  }

  async updateDashboard() {
    try {
      const data = await this.api.getStats();

      // Mise à jour des stats
      this.statsGrid.update(data);

      // Mise à jour des graphiques
      this.charts.species.update(data.species_stats);
      this.charts.performance.update(data.best_performers);
      this.charts.diversity.update(data.global_diversity);

      // Mise à jour timeline et historique
      this.updateHistory(data);
      this.charts.timeline.update(this.data.speciesHistory);

      // Mise à jour liste des espèces dans les contrôles
      this.controlPanel.updateSpeciesList(Object.keys(data.species_stats));
    } catch (error) {
      console.error('Erreur mise à jour dashboard:', error);
    }
  }

  updateHistory(data) {
    // Mise à jour historique diversité
    this.data.diversityHistory.push(data.global_diversity);
    if (this.data.diversityHistory.length > 100) {
      this.data.diversityHistory = this.data.diversityHistory.slice(-100);
    }

    // Mise à jour historique performances
    const bestScore = Math.max(...data.best_performers.map((p) => p[1]));
    this.data.performanceHistory.push(bestScore);
    if (this.data.performanceHistory.length > 100) {
      this.data.performanceHistory = this.data.performanceHistory.slice(-100);
    }

    // Mise à jour historique espèces
    const timestamp = new Date().toISOString();
    Object.entries(data.species_stats).forEach(([species, stats]) => {
      if (!this.data.speciesHistory.has(species)) {
        this.data.speciesHistory.set(species, []);
      }
      this.data.speciesHistory.get(species).push({
        time: timestamp,
        population: stats.population_size,
      });
    });
  }

  startPolling() {
    setInterval(() => this.updateDashboard(), 2000);
    this.updateDashboard();
  }
}

// Démarrage de l'application
document.addEventListener('DOMContentLoaded', () => {
  window.app = new App();
});
