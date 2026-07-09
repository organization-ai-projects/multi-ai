class API {
  constructor(baseUrl = 'http://localhost:3000') {
    this.baseUrl = baseUrl;
  }

  async getStats() {
    const response = await fetch(`${this.baseUrl}/api/stats`);
    if (!response.ok) throw new Error('Erreur récupération stats');
    return response.json();
  }

  async forceMutation() {
    try {
      const response = await fetch(`${this.baseUrl}/api/force-mutation`, {
        method: 'POST',
      });
      const data = await response.json();
      if (data.status === 'success') {
        window.logPanel.addEntry('mutation', data.message);
      } else {
        window.logPanel.addEntry('error', data.message);
      }
    } catch (error) {
      window.logPanel.addEntry('error', 'Erreur mutation forcée');
    }
  }

  async resetPopulation() {
    if (!confirm('Réinitialiser toute la population ?')) return;

    try {
      const response = await fetch(`${this.baseUrl}/api/reset`, {
        method: 'POST',
      });
      const data = await response.json();
      window.logPanel.addEntry(data.status === 'success' ? 'mutation' : 'error', data.message);
    } catch (error) {
      window.logPanel.addEntry('error', 'Erreur reset population');
    }
  }

  async saveGeneration() {
    try {
      const response = await fetch(`${this.baseUrl}/api/save-generation`, {
        method: 'POST',
      });
      const data = await response.json();
      window.logPanel.addEntry(
        data.status === 'success' ? 'mutation' : 'error',
        `Sauvegarde: ${data.path || 'erreur'}`
      );
    } catch (error) {
      window.logPanel.addEntry('error', 'Erreur sauvegarde génération');
    }
  }

  async boostSpecies(species, factor) {
    try {
      const response = await fetch(`${this.baseUrl}/api/control/species-boost`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ species, boost_factor: factor }),
      });
      const data = await response.json();
      window.logPanel.addEntry('mutation', data.message);
    } catch (error) {
      window.logPanel.addEntry('error', 'Erreur boost espèce');
    }
  }
}
