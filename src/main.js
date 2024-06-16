const canvas = document.getElementById('particleCanvas');
const ctx = canvas.getContext('2d');
canvas.width = canvas.parentElement.clientWidth;
canvas.height = canvas.parentElement.clientHeight;

let particles = {};

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

  for (const color in particles) {
      particles[color].forEach(particle => {
          ctx.fillStyle = particle.color;
          ctx.beginPath();
          ctx.arc(particle.x, particle.y, 2.57, 0, Math.PI * 2);
          ctx.fill();
      });
  }
  // requestAnimationFrame(drawParticles);
}

async function update() {
  try {
      // ctx.clearRect(0, 0, canvas.width, canvas.height);
      // Apply rules and get updated particles from Rust backend
      particles = await window.__TAURI__.invoke('update_particles');

      console.log("updating")
      
      // Redraw particles on the canvas
      drawParticles();

      // Request the next animation frame
      requestAnimationFrame(update);
  } catch (error) {
      console.error('Error updating particles:', error);
  }
}

initializeParticles();
update();