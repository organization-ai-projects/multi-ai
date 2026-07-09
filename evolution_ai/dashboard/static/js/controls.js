async function forceMutation() {
  try {
    await fetch('http://localhost:3000/api/force-mutation', { method: 'POST' });
    addLogEntry('mutation', 'Mutation forcée demandée');
  } catch (error) {
    console.error('Erreur mutation forcée:', error);
  }
}

async function resetPopulation() {
  if (confirm('Réinitialiser toute la population ?')) {
    try {
      await fetch('http://localhost:3000/api/reset', { method: 'POST' });
      addLogEntry('mutation', 'Réinitialisation population demandée');
    } catch (error) {
      console.error('Erreur reset:', error);
    }
  }
}

async function saveGeneration() {
  try {
    const response = await fetch('http://localhost:3000/api/save-generation', {
      method: 'POST',
    });
    const data = await response.json();
    addLogEntry('mutation', `Génération sauvegardée: ${data.path}`);
  } catch (error) {
    console.error('Erreur sauvegarde:', error);
  }
}

function filterSpecies() {
  const selected = document.getElementById('speciesSelect').value;
  refreshTimelineChart(selected);
}
