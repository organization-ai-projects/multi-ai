/**
 * Composant pour gérer et afficher la liste des services
 */
class ServiceList {
  /**
   * @param {string} elementId - ID de l'élément contenant la liste des services
   * @param {Api} api - Instance de l'API
   * @param {ErrorHandler} errorHandler - Gestionnaire d'erreurs
   */
  constructor(elementId, api, errorHandler) {
    console.log(`ServiceList: Initialisation pour ${elementId}`);

    this.container = document.getElementById(elementId + '-list');
    if (!this.container) {
      console.error(`ServiceList: Élément #${elementId}-list non trouvé dans le DOM`);
    }

    this.sectionElement = document.getElementById(elementId + '-section');
    if (!this.sectionElement) {
      console.error(`ServiceList: Élément #${elementId}-section non trouvé dans le DOM`);
    }

    this.api = api;
    if (!this.api) {
      console.error(`ServiceList: API non fournie`);
    }

    this.errorHandler = errorHandler;
    if (!this.errorHandler) {
      console.error(`ServiceList: ErrorHandler non fourni`);
    }

    // Liste des services connus (selon le code côté serveur)
    this.knownServices = [
      { name: 'ide_web', description: 'Environnement de développement intégré Web' },
      { name: 'ai_assistant', description: "Assistant IA pour l'aide à la programmation" },
      { name: 'version_watcher', description: 'Surveillance des versions des dépendances' },
      { name: 'semver_planner', description: 'Planificateur de versionnage sémantique' },
    ];

    // Définition des dépendances entre services (reflétant la structure du backend)
    this.dependencies = {
      ide_web: ['ai_assistant', 'version_watcher'],
      ai_assistant: ['semver_planner'],
      version_watcher: ['semver_planner'],
      semver_planner: [],
    };

    // Services qui dépendent d'un service donné (calculé à partir des dépendances)
    this.dependents = this.calculateDependents();
  }

  /**
   * Calcule les services dépendants pour chaque service
   * @returns {Object} Map des services dépendants
   */
  calculateDependents() {
    const dependents = {
      ide_web: [],
      ai_assistant: ['ide_web'],
      version_watcher: ['ide_web'],
      semver_planner: ['ai_assistant', 'version_watcher'],
    };
    return dependents;
  }

  /**
   * Récupère et affiche la liste des services
   */
  async fetchServices() {
    try {
      this.container.innerHTML = '<p>Chargement des services...</p>';

      const runningServices = await this.api.getServices();

      // Créer une map pour un accès rapide à l'état de chaque service
      const serviceStatusMap = new Map();
      runningServices.forEach((service) => {
        serviceStatusMap.set(service.name, service.running);
      });

      // Vider la liste
      this.container.innerHTML = '';

      // Titre de section mis à jour
      this.sectionElement.querySelector('h2').textContent = `Services (${
        runningServices.filter((s) => s.running).length
      } actif${runningServices.filter((s) => s.running).length > 1 ? 's' : ''})`;

      // Afficher tous les services connus, avec leur état s'il est disponible
      this.knownServices.forEach((serviceInfo) => {
        const isRunning = serviceStatusMap.has(serviceInfo.name)
          ? serviceStatusMap.get(serviceInfo.name)
          : false;

        const serviceData = {
          name: serviceInfo.name,
          description: serviceInfo.description,
          running: isRunning,
          dependencies: this.dependencies[serviceInfo.name],
          dependents: this.dependents[serviceInfo.name],
        };

        const serviceElement = this.createServiceElement(serviceData);
        this.container.appendChild(serviceElement);
      });

      this.errorHandler.hideError();
    } catch (error) {
      this.errorHandler.showError(`Erreur lors du chargement des services: ${error.message}`);
      this.container.innerHTML = '<p>Impossible de charger les services</p>';
    }
  }

  /**
   * Crée un élément DOM pour un service
   * @param {Object} service - Données du service
   * @returns {HTMLElement} Élément DOM représentant le service
   */
  createServiceElement(service) {
    // Création du HTML du composant
    const serviceDiv = document.createElement('div');
    serviceDiv.className = 'service';
    serviceDiv.dataset.status = service.running ? 'running' : 'stopped';
    serviceDiv.dataset.service = service.name;

    const serviceInfo = document.createElement('div');
    serviceInfo.className = 'service-info';

    const nameSpan = document.createElement('span');
    nameSpan.className = 'service-name';
    nameSpan.textContent = service.name;

    const descSpan = document.createElement('span');
    descSpan.className = 'service-description';
    descSpan.textContent = service.description || '';

    // Ajouter les informations de dépendance
    const dependencyInfo = document.createElement('div');
    dependencyInfo.className = 'dependency-info';

    if (service.dependencies && service.dependencies.length > 0) {
      const dependsOnSpan = document.createElement('span');
      dependsOnSpan.className = 'depends-on';
      dependsOnSpan.innerHTML = `<strong>Dépend de:</strong> ${service.dependencies.join(', ')}`;
      dependencyInfo.appendChild(dependsOnSpan);
    }

    if (service.dependents && service.dependents.length > 0) {
      const dependentsSpan = document.createElement('span');
      dependentsSpan.className = 'dependents';
      dependentsSpan.innerHTML = `<strong>Utilisé par:</strong> ${service.dependents.join(', ')}`;
      dependencyInfo.appendChild(dependentsSpan);
    }

    const statusIndicator = document.createElement('span');
    statusIndicator.className = 'status-indicator';
    statusIndicator.title = service.running ? "En cours d'exécution" : 'Arrêté';

    const button = document.createElement('button');
    button.className = service.running ? 'stop' : 'start';
    button.textContent = service.running ? 'Arrêter' : 'Démarrer';
    button.onclick = () => this.toggleService(service.name, service.running);

    serviceInfo.appendChild(nameSpan);
    serviceInfo.appendChild(descSpan);
    serviceInfo.appendChild(dependencyInfo);

    serviceDiv.appendChild(statusIndicator);
    serviceDiv.appendChild(serviceInfo);
    serviceDiv.appendChild(button);

    return serviceDiv;
  }

  /**
   * Démarre ou arrête un service
   * @param {string} name - Nom du service
   * @param {boolean} running - État actuel du service
   */
  async toggleService(name, running) {
    try {
      const button = this.container.querySelector(`.service[data-service="${name}"] button`);
      if (button) {
        button.disabled = true;
        button.textContent = running ? 'Arrêt...' : 'Démarrage...';
      }

      // Afficher un message d'information sur les dépendances
      if (!running) {
        const dependencies = this.dependencies[name];
        if (dependencies && dependencies.length > 0) {
          this.errorHandler.showInfo(
            `Démarrage de ${name}... Les dépendances (${dependencies.join(
              ', '
            )}) seront également démarrées si nécessaire.`
          );
        }
      } else {
        const dependents = this.dependents[name];
        if (dependents && dependents.length > 0) {
          this.errorHandler.showInfo(
            `Arrêt de ${name}... Les services dépendants (${dependents.join(
              ', '
            )}) seront également arrêtés.`
          );
        }
      }

      const result = await this.api.toggleService(name, running);

      if (result.success) {
        this.fetchServices();
      } else {
        if (button) {
          button.disabled = false;
          button.textContent = running ? 'Arrêter' : 'Démarrer';
        }
        this.errorHandler.showError(`Échec de l'opération sur le service ${name}`);
      }
    } catch (error) {
      this.errorHandler.showError(
        `Erreur lors de l'interaction avec le service ${name}: ${error.message}`
      );
      const button = this.container.querySelector(`.service[data-service="${name}"] button`);
      if (button) {
        button.disabled = false;
        button.textContent = running ? 'Arrêter' : 'Démarrer';
      }
    }
  }

  /**
   * Nettoie les ressources lors de la destruction du composant
   */
  destroy() {
    // Nettoyage si nécessaire
  }
}
