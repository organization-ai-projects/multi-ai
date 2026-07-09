class ControlPanel {
  constructor(api) {
    this.api = api;
    this.container = document.getElementById('controls-panel');
    this.init();
  }

  init() {
    this.container.className = 'control-panel';
    this.container.innerHTML = `
            <h3>Contrôles</h3>
            <button class="control-button" id="mutate">Forcer Mutation</button>
            <button class="control-button" id="reset">Reset Population</button>
            <button class="control-button" id="save">Sauvegarder Génération</button>
            
            <div class="species-filter">
                <select id="speciesSelect">
                    <option value="all">Toutes les espèces</option>
                </select>
                <input type="number" id="generationRange" value="50" 
                       placeholder="Générations à afficher">
            </div>
        `;

    this.bindEvents();
  }

  bindEvents() {
    document.getElementById('mutate').onclick = () => this.api.forceMutation();
    document.getElementById('reset').onclick = () => this.api.resetPopulation();
    document.getElementById('save').onclick = () => this.api.saveGeneration();
    document.getElementById('speciesSelect').onchange = (e) =>
      this.api.filterSpecies(e.target.value);
  }

  updateSpeciesList(species) {
    const select = document.getElementById('speciesSelect');
    const current = select.value;

    select.innerHTML = '<option value="all">Toutes les espèces</option>';
    species.forEach((s) => {
      const opt = document.createElement('option');
      opt.value = s;
      opt.textContent = s;
      select.appendChild(opt);
    });

    select.value = current;
  }
}
