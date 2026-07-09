/**
 * Vue principale affichant la liste des services
 */
class HomeView {
  constructor() {
    console.log('HomeView: Initialisation');
    this.template = document.getElementById('home-template');
    if (!this.template) {
      console.error('HomeView: Template non trouvé');
    }
    this.components = {};
    this.refreshInterval = null;
  }

  /**
   * Monte la vue dans le conteneur spécifié
   * @param {HTMLElement} container - Élément conteneur
   */
  mount(container) {
    console.log('HomeView: Montage dans le conteneur');

    // Cloner le template
    const content = document.importNode(this.template.content, true);
    container.appendChild(content);

    try {
      // Initialiser les composants
      this.initComponents();

      // Charger les données
      this.loadData();

      // Configurer le rafraîchissement automatique
      this.refreshInterval = setInterval(() => {
        if (this.components.serviceList) {
          this.components.serviceList.fetchServices();
        }
      }, 10000);

      console.log('HomeView: Vue montée avec succès');
    } catch (error) {
      console.error('HomeView: Erreur lors du montage', error);
      container.innerHTML = `<div class="error-message" style="display:block">Erreur lors du chargement de la vue: ${error.message}</div>`;
    }
  }

  /**
   * Initialise les composants de la vue
   */
  initComponents() {
    console.log('HomeView: Initialisation des composants');

    // Obtenir l'API de l'instance globale de l'application
    const api = window.app ? window.app.getApi() : new Api();

    if (!api) {
      throw new Error("L'API n'est pas disponible");
    }

    // Créer les instances des composants
    this.components.errorHandler = new ErrorHandler('error-message');
    this.components.header = new Header('header', api);
    this.components.serviceList = new ServiceList('services', api, this.components.errorHandler);

    console.log('HomeView: Composants initialisés');
  }

  /**
   * Charge les données initiales
   */
  loadData() {
    console.log('HomeView: Chargement des données');
    if (this.components.serviceList) {
      this.components.serviceList.fetchServices();
    } else {
      console.error('HomeView: ServiceList non initialisé');
    }
  }

  /**
   * Nettoie la vue lors de sa destruction
   */
  destroy() {
    console.log('HomeView: Destruction');
    // Arrêter le rafraîchissement automatique
    if (this.refreshInterval) {
      clearInterval(this.refreshInterval);
    }

    // Nettoyer les composants si nécessaire
    Object.values(this.components).forEach((component) => {
      if (component && typeof component.destroy === 'function') {
        component.destroy();
      }
    });
  }
}
