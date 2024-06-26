const canvas = document.getElementById('particleCanvas');
const ctx = canvas.getContext('2d');
setupCanvas();

let particles = {};

function setupCanvas() {
    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.getBoundingClientRect();
    
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
    canvas.style.width = `${rect.width}px`;
    canvas.style.height = `${rect.height}px`;
    
    ctx.scale(dpr, dpr);
}

async function initializeParticles() {
  console.log("In init.")
    try {
        console.log("Initializing particles...");
        particles = await window.__TAURI__.invoke('init_particles');
        console.log("Particles initialized:", particles);
        drawParticles();
    } catch (error) {
        console.error('Error initializing particles:', error);
    }
}

function drawParticles() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.imageSmoothingEnabled = false; 

  for (const color in particles) {
      particles[color].forEach(particle => {
          ctx.fillStyle = particle.color;
          ctx.beginPath();
          ctx.arc(particle.x, particle.y, 1, 0, Math.PI * 2);
          ctx.fill();
      });
  }
}

async function update() {
  try {
      particles = await window.__TAURI__.invoke('update_particles');
      
      drawParticles();

      requestAnimationFrame(update);
  } catch (error) {
      console.error('Error updating particles:', error);
  }
}

async function updateNumAtoms(num) {
    try {
        particles = await window.__TAURI__.invoke('update_num_atoms', { newNum: num });
        console.log('Number of atoms updated successfully');
        drawParticles(); // Redraw particles with new count
    } catch (error) {
        console.error('Error updating number of atoms:', error);
    }
}

async function updateSettings() {
    const newNumAtoms = parseInt(document.getElementById('numAtoms').value);
    const currentNumAtoms = Object.values(particles)[0].length;

    if (newNumAtoms !== currentNumAtoms) {
        // Only update number of atoms if it has changed
        await updateNumAtoms(newNumAtoms);
    }

    const newRules = {rules: {}};
    colors.forEach(color => {
        newRules.rules[color] = {};
    });

    colors.forEach(color1 => {
        colors.forEach(color2 => {
            const sliderId = `${color1}-${color2}-interaction`;
            const value = parseFloat(document.getElementById(sliderId).value);
            newRules.rules[color1][color2] = value;
        });
    });

    try {
        // Use the new command that only updates rules
        await window.__TAURI__.invoke('update_rules_only', {newRules});
        console.log('Rules updated successfully');
    } catch (error) {
        console.error('Error updating rules:', error);
    }
}

// async function updateSettings() {
//     const newNumAtoms = parseInt(document.getElementById('numAtoms').value);
//     await updateNumAtoms(newNumAtoms);

//     const newRules = {rules: {}};
//     colors.forEach(color => {
//         newRules.rules[color] = {};
//     });

//     colors.forEach(color1 => {
//         colors.forEach(color2 => {
//             const sliderId = `${color1}-${color2}-interaction`;
//             const value = parseFloat(document.getElementById(sliderId).value);
//             newRules.rules[color1][color2] = value;
//         });
//     });

//     try {
//         await window.__TAURI__.invoke('update_interaction_rules', {newRules});
//         console.log('Rules updated successfully');
//         initializeRules(); // Refresh UI
//     } catch (error) {
//         console.error('Error updating rules:', error);
//     }
// }

initializeParticles();
update();

const colors = ['red', 'magenta', 'green', 'yellow'];
const interactionRulesContainer = document.getElementById('interactionRules');

colors.forEach(color1 => {
    colors.forEach(color2 => {
        const sliderId = `${color1}-${color2}-interaction`;
        const sliderHtml = `
            <div class="form-group">
                <label for="${sliderId}">${color1.charAt(0).toUpperCase() + color1.slice(1)} → ${color2.charAt(0).toUpperCase() + color2.slice(1)}:</label>
                <input type="range" id="${sliderId}" name="${sliderId}" min="-1" max="1" step="0.0001" value="0">
                <span id="${sliderId}-value">0</span>
            </div>
        `;
        interactionRulesContainer.innerHTML += sliderHtml;
    });
});

document.querySelectorAll('input[type="range"]').forEach(slider => {
    const valueDisplay = document.getElementById(`${slider.id}-value`);
    slider.addEventListener('input', () => {
        valueDisplay.textContent = slider.value;
    });
});

async function initializeRules() {
    try {
        const rules = await window.__TAURI__.invoke('get_interaction_rules');
        colors.forEach((color1, i) => {
            colors.slice(i).forEach(color2 => {
                const sliderId = `${color1}-${color2}-interaction`;
                const slider = document.getElementById(sliderId);
                const value = rules.rules[color1][color2];
                slider.value = value;
                document.getElementById(`${sliderId}-value`).textContent = value.toFixed(1);
            });
        });
    } catch (error) {
        console.error('Error initializing rules:', error);
    }
}

initializeRules();