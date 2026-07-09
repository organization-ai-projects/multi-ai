/**
 * Composant pour gérer et afficher les erreurs et informations
 */
class ErrorHandler {
  constructor(elementId = 'error-message') {
    this.errorElement = document.getElementById(elementId);
    this.messageTimeout = null;
  }

  /**
   * Affiche un message d'erreur
   * @param {string} message - Le message d'erreur à afficher
   */
  showError(message) {
    this.clearTimeout();
    this.errorElement.textContent = message;
    this.errorElement.className = 'error-message';
    this.errorElement.style.display = 'block';
    console.error(message);
  }

  /**
   * Affiche un message d'information
   * @param {string} message - Le message d'information à afficher
   * @param {number} timeout - Durée d'affichage en ms (0 pour pas de timeout)
   */
  showInfo(message, timeout = 5000) {
    this.clearTimeout();
    this.errorElement.textContent = message;
    this.errorElement.className = 'info-message';
    this.errorElement.style.display = 'block';
    console.info(message);

    if (timeout > 0) {
      this.messageTimeout = setTimeout(() => this.hideError(), timeout);
    }
  }

  /**
   * Cache le message
   */
  hideError() {
    this.clearTimeout();
    this.errorElement.style.display = 'none';
  }

  /**
   * Nettoie le timeout existant
   */
  clearTimeout() {
    if (this.messageTimeout) {
      clearTimeout(this.messageTimeout);
      this.messageTimeout = null;
    }
  }

  /**
   * Nettoie les ressources lors de la destruction du composant
   */
  destroy() {
    this.clearTimeout();
  }
}
