export function startMatrixAnimation(canvasId) {
  const canvas = document.getElementById(canvasId);
  const ctx = canvas.getContext('2d');

  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;

  const chars = '0123456789/*-+/<>?;:[]~!@#$%^&*()+=abcdefghijklmnopqrstuvwxyz';
  const fontSize = 16;
  const columns = canvas.width / fontSize;
  const drops = Array.from({ length: columns }, () => 0);

  // Initialisiere Skalierungsfaktoren
  const scales = Array.from({ length: columns }, () => 1);
  const scaleDirection = Array.from({ length: columns }, () => 1);

  function draw() {
      ctx.fillStyle = 'rgba(0, 0, 0, 0.05)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      drops.forEach((drop, x) => {
          const text = chars[Math.floor(Math.random() * chars.length)];
          
          // Dynamische Skalierung
          let scale = scales[x];
          scale += scaleDirection[x] * 0.05;
          if (scale > 1.8 || scale < 1) {
              scaleDirection[x] *= -1; // Ändere die Richtung der Skalierung
          }
          scales[x] = scale;

          ctx.font = `${fontSize * scale}px monospace`;
          ctx.fillStyle = 'green';
          ctx.fillText(text, x * fontSize, drop * fontSize);

          if (drop * fontSize > canvas.height || Math.random() > 0.97) {
              drops[x] = 0;
          } else {
              drops[x]++;
          }
      });
  }

  setInterval(draw, 80); // 80ms Verzögerung zwischen den Frames (12.5fps)
}