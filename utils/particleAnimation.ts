
export interface ParticleAnimationOptions {
  /** Number of particles */
  particleCount?: number;
  /** Particle color (RGBA format) */
  color?: string;
  connect?: boolean;
  /** Minimum particle size (pixels) */
  minSize?: number;
  /** Maximum particle size (pixels) */
  maxSize?: number;
  /** Minimum particle speed */
  minSpeed?: number;
  /** Maximum particle speed */
  maxSpeed?: number;
  /** Maximum distance for particle connections */
  connectionDistance?: number;
  /** Connection line opacity */
  connectionOpacity?: number;
}

/**
 * Initialize moving particle animation on grid background
 */

  // Create particles

    // Set styles
    particle.style.cssText = `
      position: absolute;
      left: ${x}%;
      top: ${y}%;
      width: ${size}px;
      height: ${size}px;
      background: ${color.replace(')', `, ${opacity})`)};
      border-radius: 50%;
      pointer-events: none;
      filter: blur(1px);
      box-shadow: 0 0 ${size * 3}px ${color.replace(')', `, ${opacity * 2})`)};
    `;
    
    // Store speed data
    particle.dataset.speedX = speedX.toString();
    particle.dataset.speedY = speedY.toString();
    
    container.appendChild(particle);
    particles.push(particle);
  }
  
  // If connection mode is enabled, create canvas for drawing lines between particles
  let canvas: HTMLCanvasElement | null = null;
  let ctx: CanvasRenderingContext2D | null = null;
  let animationIdCanvas: number | null = null;
  let updateCanvasSize: (() => void) | null = null;
  
  if (connect) {
    canvas = document.createElement('canvas');
    canvas.style.cssText = `
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      pointer-events: none;
    `;
    container.appendChild(canvas);
    
    ctx = canvas.getContext('2d');
    
    // Set canvas size and update on resize
    updateCanvasSize = () => {
      if (!canvas) return;
      const rect = container.getBoundingClientRect();
      canvas.width = rect.width;
      canvas.height = rect.height;
    };
    
    if (updateCanvasSize) {
      updateCanvasSize();
      window.addEventListener('resize', updateCanvasSize);
    }
  }
  

    for (let i = 0; i < particleCount; i++) {
    const particle = document.createElement('div');
    particle.classList.add('moving-dot');
    
    const x = Math.random() * 100;
    const y = Math.random() * 100;
    
    // Random speed (slow movement)
    const speedX = (Math.random() - 0.5) * (maxSpeed - minSpeed) + minSpeed;
    const speedY = (Math.random() - 0.5) * (maxSpeed - minSpeed) + minSpeed;
    
    // Random size and opacity
    const size = Math.random() * (maxSize - minSize) + minSize;
    const opacity = Math.random() * 0.4 + 0.1;
    
  // Animation function
  let animationId: number | null = null;
  
  const animateParticles = () => {
    particles.forEach(particle => {
      // Get current position
      let x = parseFloat(particle.style.left);
      let y = parseFloat(particle.style.top);
      
      // Get speed
      const speedX = parseFloat(particle.dataset.speedX || '0');
      const speedY = parseFloat(particle.dataset.speedY || '0');
      
      // Update position
      x = (x + speedX) % 100;
      y = (y + speedY) % 100;
      
      if (x < 0) x = 100 + x;
      if (y < 0) y = 100 + y;
      
      // Apply new position
      particle.style.left = `${x}%`;
      particle.style.top = `${y}%`;
    });
    
    // If connection is enabled, draw lines between particles
    if (connect && ctx && canvas) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      
      // Draw lines for each pair of particles that are close enough
      for (let i = 0; i < particles.length; i++) {
        const p1 = particles[i];
        const p1Rect = p1.getBoundingClientRect();
        const containerRect = container.getBoundingClientRect();
        
        // Calculate relative position
        const x1 = p1Rect.left + p1Rect.width / 2 - containerRect.left;
        const y1 = p1Rect.top + p1Rect.height / 2 - containerRect.top;
        
        for (let j = i + 1; j < particles.length; j++) {
          const p2 = particles[j];
          const p2Rect = p2.getBoundingClientRect();
          
          // Calculate relative position
          const x2 = p2Rect.left + p2Rect.width / 2 - containerRect.left;
          const y2 = p2Rect.top + p2Rect.height / 2 - containerRect.top;
          
          // Calculate distance
          const dx = x1 - x2;
          const dy = y1 - y2;
          const distance = Math.sqrt(dx * dx + dy * dy);
          
          // If particles are close enough, draw connection line
          const maxDistance = connectionDistance || Math.min(canvas.width, canvas.height) / 4;
          if (distance < maxDistance) {
            // Calculate opacity based on distance
            const opacity = connectionOpacity * (1 - distance / maxDistance);
            ctx.strokeStyle = color.replace(')', `, ${opacity})`);
            ctx.lineWidth = 0.5;
            ctx.beginPath();
            ctx.moveTo(x1, y1);
            ctx.lineTo(x2, y2);
            ctx.stroke();
          }
        }
      }
      
      animationIdCanvas = requestAnimationFrame(animateParticles);
    } else {
      animationId = requestAnimationFrame(animateParticles);
    }
  };
  
  animateParticles();
  
  // Return cleanup function
  return () => {
    if (animationId) cancelAnimationFrame(animationId);
    if (animationIdCanvas) cancelAnimationFrame(animationIdCanvas);
    if (connect && updateCanvasSize) window.removeEventListener('resize', updateCanvasSize);
    
    // Remove all particles and canvas
    particles.forEach(particle => {
      if (particle.parentNode) {
        particle.parentNode.removeChild(particle);
      }
    });
    
    if (canvas && canvas.parentNode) {
      canvas.parentNode.removeChild(canvas);
    }
  };
}

// Export module
export { initParticleAnimation }; 