/**
 * Point d'entrée principal de l'application SPA
 */
class App {
  constructor() {
    console.log("App: Construction de l'instance");
    this.router = new Router();
    this.api = new Api();
  }

  /**
   * Initialise l'application
   */
  init() {
    console.log('App: Initialisation...');

    // Configurer les routes
    this.router.addRoute('/', HomeView);

    // Démarrer le routeur
    console.log("App: Navigation vers la page d'accueil");
    this.router.navigateTo('/');

    console.log('App: Application Workspace Manager initialisée');
  }

  /**
   * Obtient l'instance de l'API
   * @returns {Api} Instance de l'API
   */
  getApi() {
    return this.api;
  }
}
