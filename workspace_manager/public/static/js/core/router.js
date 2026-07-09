/**
 * Routeur simple pour gérer les différentes vues de l'application SPA
 */
class Router {
  constructor() {
    this.routes = {};
    this.currentView = null;
    this.appElement = document.getElementById('app');

    // Gérer la navigation
    window.addEventListener('popstate', this.handleRouteChange.bind(this));
  }

  /**
   * Ajoute une route
   * @param {string} path - Chemin de la route
   * @param {class} viewClass - Classe de vue à instancier
   */
  addRoute(path, viewClass) {
    this.routes[path] = viewClass;
  }

  /**
   * Gère les changements de route
   */
  handleRouteChange() {
    const path = window.location.pathname || '/';
    this.navigateTo(path);
  }

  /**
   * Navigue vers une route spécifique
   * @param {string} path - Chemin de destination
   */
  navigateTo(path) {
    // Nettoyer la vue actuelle si elle existe
    if (this.currentView && typeof this.currentView.destroy === 'function') {
      this.currentView.destroy();
    }

    // Trouver la vue correspondante ou utiliser la vue par défaut
    const ViewClass = this.routes[path] || this.routes['/'];

    if (!ViewClass) {
      console.error(`Aucune vue trouvée pour le chemin: ${path}`);
      return;
    }

    // Nettoyer le conteneur
    this.appElement.innerHTML = '';

    // Instancier et monter la nouvelle vue
    this.currentView = new ViewClass();
    this.currentView.mount(this.appElement);

    // Mettre à jour l'historique si nécessaire
    if (window.location.pathname !== path) {
      window.history.pushState({}, '', path);
    }
  }
}
