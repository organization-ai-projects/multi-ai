/**
 * Classe pour gérer les appels API
 */
class Api {
  /**
   * Récupère la liste des services
   * @returns {Promise<Array>} Liste des services
   */
  async getServices() {
    const response = await fetch('/api/services');
    if (!response.ok) {
      throw new Error(`Erreur HTTP: ${response.status}`);
    }
    const services = await response.json();

    // Vérifier si la réponse est un tableau
    if (!Array.isArray(services)) {
      throw new Error('Format de réponse invalide');
    }

    return services;
  }

  /**
   * Démarre ou arrête un service
   * @param {string} name - Nom du service
   * @param {boolean} running - État actuel du service
   * @returns {Promise<Object>} Résultat de l'opération
   */
  async toggleService(name, running) {
    const endpoint = running ? `/api/service/stop/${name}` : `/api/service/start/${name}`;
    const response = await fetch(endpoint, { method: 'POST' });

    if (!response.ok) {
      throw new Error(`Erreur HTTP: ${response.status}`);
    }

    return await response.json();
  }

  /**
   * Envoie une commande à un service spécifique
   * @param {string} service - Nom du service
   * @param {string} command - Commande à envoyer
   * @returns {Promise<Object>} Résultat de l'opération
   */
  async sendCommand(service, command) {
    const response = await fetch(`/api/command/${service}/${command}`, {
      method: 'POST',
    });

    if (!response.ok) {
      throw new Error(`Erreur HTTP: ${response.status}`);
    }

    return await response.json();
  }
}
