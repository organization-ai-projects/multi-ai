/**
 * Composant pour le header de l'application
 */
class Header {
  /**
   * @param {string} elementId - ID de l'élément contenant le header
   * @param {Api} api - Instance de l'API
   */
  constructor(elementId, api) {
    this.container = document.getElementById(elementId);
    this.api = api;
    this.render();
  }

  /**
   * Affiche le header avec les contrôles nécessaires
   */
  render() {
    // Titre principal
    const title = document.createElement('h1');
    title.textContent = 'Workspace Manager';

    // Sous-titre
    const subtitle = document.createElement('span');
    subtitle.className = 'subtitle';
    subtitle.textContent = 'Gestion des services Multi-AI';

    this.container.appendChild(title);
    this.container.appendChild(subtitle);
  }
}
