class LogPanel {
  constructor() {
    this.container = document.getElementById('logs-panel');
    this.logHistory = [];
    this.init();
  }

  init() {
    this.container.className = 'control-panel';
    this.container.innerHTML = `
            <h3>Logs d'évolution</h3>
            <div id="logPanel" class="log-panel"></div>
        `;
  }

  addEntry(type, message) {
    const entry = {
      type,
      message,
      timestamp: new Date(),
    };

    this.logHistory.push(entry);
    this.renderEntry(entry);
  }

  renderEntry(entry) {
    const panel = document.getElementById('logPanel');
    const div = document.createElement('div');
    div.className = `log-entry log-${entry.type}`;
    div.textContent = `[${entry.timestamp.toLocaleTimeString()}] ${entry.message}`;
    panel.insertBefore(div, panel.firstChild);
  }
}
