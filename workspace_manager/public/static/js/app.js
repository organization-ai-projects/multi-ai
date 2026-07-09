/**
 * Point d'entrée de l'application
 * Initialise tous les composants et gère leur coordination
 */
document.addEventListener('DOMContentLoaded', () => {
  // Créer les instances des composants principaux
  const api = new Api();
  const errorHandler = new ErrorHandler('error-message');
  const header = new Header('header', api);
  const serviceList = new ServiceList('services', api, errorHandler);

  // Charger les données initiales
  serviceList.fetchServices();

  // Rafraîchir périodiquement la liste des services
  setInterval(() => {
    serviceList.fetchServices();
  }, 10000); // Toutes les 10 secondes
});
