let diversityHistory = [];
let performanceHistory = [];
let speciesChart, performanceChart, diversityChart, speciesTimelineChart;

function initCharts() {
  const commonOptions = {
    responsive: true,
    animation: false,
    plugins: {
      legend: { position: 'top' },
    },
  };

  // Graphique des espèces (populations actuelles)
  speciesChart = new Chart(document.getElementById('speciesChart'), {
    type: 'bar',
    data: {
      labels: [],
      datasets: [
        {
          label: 'Population par espèce',
          data: [],
          backgroundColor: [
            'rgba(255, 99, 132, 0.5)',
            'rgba(54, 162, 235, 0.5)',
            'rgba(255, 206, 86, 0.5)',
            'rgba(75, 192, 192, 0.5)',
          ],
        },
      ],
    },
    options: {
      ...commonOptions,
      plugins: {
        ...commonOptions.plugins,
        title: {
          display: true,
          text: 'Distribution des espèces',
        },
      },
      scales: {
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: "Nombre d'individus",
          },
        },
      },
    },
  });

  // Graphique des performances (historique)
  performanceChart = new Chart(document.getElementById('performanceChart'), {
    type: 'line',
    data: {
      labels: [],
      datasets: [
        {
          label: 'Meilleure performance',
          data: [],
          borderColor: 'rgb(75, 192, 192)',
          tension: 0.1,
          fill: false,
        },
      ],
    },
    options: {
      ...commonOptions,
      plugins: {
        ...commonOptions.plugins,
        title: {
          display: true,
          text: 'Évolution des performances',
        },
      },
      scales: {
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: 'Score',
          },
        },
        x: {
          title: {
            display: true,
            text: 'Génération',
          },
        },
      },
    },
  });

  // Graphique de diversité (ligne)
  diversityChart = new Chart(document.getElementById('diversityChart'), {
    type: 'line',
    data: {
      labels: [],
      datasets: [
        {
          label: 'Diversité de la population',
          data: [],
          borderColor: 'rgb(255, 99, 132)',
          tension: 0.1,
          fill: false,
        },
      ],
    },
    options: {
      ...commonOptions,
      plugins: {
        ...commonOptions.plugins,
        title: {
          display: true,
          text: 'Évolution de la diversité',
        },
      },
      scales: {
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: 'Indice de diversité',
          },
        },
        x: {
          title: {
            display: true,
            text: 'Génération',
          },
        },
      },
    },
  });

  // Timeline des espèces (stacked area)
  initSpeciesTimeline();
}

function initSpeciesTimeline() {
  const colorPalette = [
    'rgba(255, 99, 132, 0.3)',
    'rgba(54, 162, 235, 0.3)',
    'rgba(255, 206, 86, 0.3)',
    'rgba(75, 192, 192, 0.3)',
    'rgba(153, 102, 255, 0.3)',
  ];

  speciesTimelineChart = new Chart(document.getElementById('speciesTimelineChart'), {
    type: 'line',
    data: {
      labels: [],
      datasets: [],
    },
    options: {
      responsive: true,
      animation: false,
      plugins: {
        title: {
          display: true,
          text: 'Évolution des populations par espèce',
        },
        tooltip: {
          mode: 'index',
          intersect: false,
        },
      },
      scales: {
        y: {
          stacked: true,
          beginAtZero: true,
          title: {
            display: true,
            text: 'Population',
          },
        },
        x: {
          title: {
            display: true,
            text: 'Temps',
          },
        },
      },
      elements: {
        line: {
          fill: true,
        },
      },
    },
  });
}

function updateSpeciesChart(stats) {
  const labels = Object.keys(stats);
  const data = labels.map((species) => stats[species].population_size);

  speciesChart.data.labels = labels;
  speciesChart.data.datasets[0].data = data;
  speciesChart.update('none'); // désactive l'animation pour la performance

  document.getElementById('totalSpecies').textContent = labels.length;
}

function updatePerformanceChart(performers) {
  if (performers.length > 0) {
    const bestScore = Math.max(...performers.map((p) => p[1]));
    performanceHistory.push(bestScore);

    // Garder un historique limité pour la performance
    if (performanceHistory.length > 100) {
      performanceHistory = performanceHistory.slice(-100);
    }

    const labels = Array.from(
      { length: performanceHistory.length },
      (_, i) => i + Math.max(0, performanceHistory.length - 100)
    );

    performanceChart.data.labels = labels;
    performanceChart.data.datasets[0].data = performanceHistory;
    performanceChart.update('none');

    document.getElementById('bestScore').textContent = bestScore.toFixed(2);
  }
}

function updateDiversityChart(diversity) {
  diversityHistory.push(diversity);

  // Garder un historique limité
  if (diversityHistory.length > 100) {
    diversityHistory = diversityHistory.slice(-100);
  }

  const labels = Array.from(
    { length: diversityHistory.length },
    (_, i) => i + Math.max(0, diversityHistory.length - 100)
  );

  diversityChart.data.labels = labels;
  diversityChart.data.datasets[0].data = diversityHistory;
  diversityChart.update('none');

  document.getElementById('diversity').textContent = diversity.toFixed(3);
}
