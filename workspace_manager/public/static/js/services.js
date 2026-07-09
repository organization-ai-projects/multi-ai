/**
 * Fonctions de service pour gérer les appels API
 */

// Récupère la liste des services
async function fetchServices() {
  try {
    const response = await fetch('/api/services');
    if (!response.ok) {
      throw new Error(`Erreur HTTP: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    showError(`Erreur lors du chargement des services: ${error.message}`);
    return [];
  }
}

// Démarre ou arrête un service
async function toggleService(name, running) {
  try {
    const endpoint = running ? `/api/service/stop/${name}` : `/api/service/start/${name}`;
    const response = await fetch(endpoint, { method: 'POST' });

    if (!response.ok) {
      throw new Error(`Erreur HTTP: ${response.status}`);
    }

    const result = await response.json();
    if (result.success) {
      updateServicesList();
    } else {
      showError(`Échec de l'opération sur le service ${name}`);
    }
  } catch (error) {
    showError(`Erreur lors de l'interaction avec le service ${name}: ${error.message}`);
  }
}

// Envoie une commande à un service spécifique
async function sendCommand(service, command) {
  try {
    const response = await fetch(`/api/command/${service}/${command}`, {
      method: 'POST',
    });

    if (!response.ok) {
      throw new Error(`Erreur HTTP: ${response.status}`);
    }

    return await response.json();
  } catch (error) {
    showError(`Erreur lors de l'envoi de la commande: ${error.message}`);
    return { success: false };
  }
}
