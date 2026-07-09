let speciesHistory = new Map();
let logHistory = [];

function addLogEntry(type, message) {
  const logPanel = document.getElementById('logPanel');
  const entry = document.createElement('div');
  entry.className = `log-entry log-${type}`;
  entry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;
  logPanel.insertBefore(entry, logPanel.firstChild);
  logHistory.push({ type, message, timestamp: new Date() });
}

async function updateDashboard() {
  try {
    const response = await fetch('http://localhost:3000/api/stats');
    const data = await response.json();

    document.getElementById('currentGen').textContent = data.current_generation;

    updateSpeciesChart(data.species_stats);
    updatePerformanceChart(data.best_performers);
    updateDiversityChart(data.global_diversity);
    updateSpeciesTimeline(data.species_stats);
  } catch (error) {
    console.error('Erreur mise à jour dashboard:', error);
  }
}
