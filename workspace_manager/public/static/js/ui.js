/**
 * Fonctions UI pour manipuler le DOM
 */

// Gestion des erreurs
function showError(message) {
  const errorElement = document.getElementById('error-message');
  errorElement.textContent = message;
  errorElement.classList.remove('hidden');
  console.error(message);
}

function hideError() {
  const errorElement = document.getElementById('error-message');
  errorElement.classList.add('hidden');
}

// Mise à jour de la liste des services
async function updateServicesList() {
  const servicesList = document.getElementById('services-list');
  servicesList.innerHTML = '<p>Chargement des services...</p>';

  const services = await fetchServices();

  if (!Array.isArray(services)) {
    showError('Format de réponse invalide');
    servicesList.innerHTML = '<p>Impossible de charger les services</p>';
    return;
  }

  if (services.length === 0) {
    servicesList.innerHTML = '<p>Aucun service disponible</p>';
    return;
  }

  // Vider la liste
  servicesList.innerHTML = '';

  // Créer un élément HTML pour chaque service
  services.forEach((service) => {
    const serviceElement = createServiceElement(service);
    servicesList.appendChild(serviceElement);
  });
}

// Création d'un élément de service
function createServiceElement(service) {
  // Création du HTML du composant
  const serviceDiv = document.createElement('div');
  serviceDiv.className = 'service';

  const nameSpan = document.createElement('span');
  nameSpan.textContent = service.name;

  const button = document.createElement('button');
  button.className = service.running ? 'stop' : 'start';
  button.textContent = service.running ? 'Arrêter' : 'Démarrer';
  button.onclick = () => toggleService(service.name, service.running);

  serviceDiv.appendChild(nameSpan);
  serviceDiv.appendChild(button);

  return serviceDiv;
}

// Initialisation
document.addEventListener('DOMContentLoaded', () => {
  // Charger les services au démarrage
  updateServicesList();

  // Rafraîchir périodiquement la liste des services
  setInterval(updateServicesList, 10000); // Toutes les 10 secondes
});
