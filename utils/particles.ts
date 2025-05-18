/**
 * particles.ts
 * For generating background particle animation effects
 */

interface Particle {
  x: number;
  y: number;
  size: number;
  speedX: number;
  speedY: number;
  color: string;
  element: HTMLDivElement;
}

interface ParticlesOptions {
  container: HTMLElement;
  count?: number;
  colors?: string[];
  minSpeed?: number;
  maxSpeed?: number;
  minSize?: number;
  maxSize?: number;
  connectParticles?: boolean;
  connectDistance?: number;
}

export class ParticlesEffect {
  private particles: Particle[] = [];
  private container: HTMLElement;
  private width: number = 0;
  private height: number = 0;
  private isAnimating: boolean = false;
  private rafId: number | null = null;
  private resizeObserver: ResizeObserver | null = null;
  
  private options = {
    count: 30,
    colors: ['rgba(0, 217, 218, 0.5)', 'rgba(123, 90, 255, 0.5)', 'rgba(255, 74, 74, 0.5)', 'rgba(77, 182, 255, 0.5)'],
    minSpeed: 0.05,
    maxSpeed: 0.2,
    minSize: 2,
    maxSize: 5,
    connectParticles: true,
    connectDistance: 150
  };

  constructor(options: ParticlesOptions) {
    this.container = options.container;
    
    // Merge configuration options
    this.options = {
      ...this.options,
      ...options
    };
    
    this.init();
  }

  private init(): void {
    this.width = this.container.offsetWidth;
    this.height = this.container.offsetHeight;
    
    // Create specified number of particles
    this.createParticles();
    
    // Listen for container size changes
    this.resizeObserver = new ResizeObserver(this.handleResize);
    this.resizeObserver.observe(this.container);
  }

  private createParticles(): void {
    // Remove existing particles
    this.removeAllParticles();
    
    // Create new particles
    for (let i = 0; i < this.options.count; i++) {
      const size = this.random(this.options.minSize, this.options.maxSize);
      const x = this.random(0, this.width);
      const y = this.random(0, this.height);
      const speedX = this.random(this.options.minSpeed, this.options.maxSpeed) * (Math.random() > 0.5 ? 1 : -1);
      const speedY = this.random(this.options.minSpeed, this.options.maxSpeed) * (Math.random() > 0.5 ? 1 : -1);
      const color = this.options.colors[Math.floor(Math.random() * this.options.colors.length)];
      
      const particleElement = document.createElement('div');
      particleElement.className = 'particle';
      particleElement.style.position = 'absolute';
      particleElement.style.width = `${size}px`;
      particleElement.style.height = `${size}px`;
      particleElement.style.backgroundColor = color;
      particleElement.style.borderRadius = '50%';
      particleElement.style.left = `${x}px`;
      particleElement.style.top = `${y}px`;
      particleElement.style.opacity = '0';
      particleElement.style.transform = 'scale(0)';
      particleElement.style.transition = 'opacity 0.8s ease, transform 0.8s ease';
      
      this.container.appendChild(particleElement);
      
      // Add to particles array
      this.particles.push({
        x,
        y,
        size,
        speedX,
        speedY,
        color,
        element: particleElement
      });
      
      // Delay particle display to create fade-in effect
      setTimeout(() => {
        particleElement.style.opacity = '1';
        particleElement.style.transform = 'scale(1)';
      }, 50 * i);
    }
  }

  private removeAllParticles(): void {
    // Remove all particle elements
    this.particles.forEach(particle => {
      if (particle.element.parentNode) {
        particle.element.parentNode.removeChild(particle.element);
      }
    });
    
    // Clear particles array
    this.particles = [];
  }

  private random(min: number, max: number): number {
    return Math.random() * (max - min) + min;
  }

  private update = (): void => {
    // Update each particle's position
    this.particles.forEach(particle => {
      // Move particle
      particle.x += particle.speedX;
      particle.y += particle.speedY;
      
      // Boundary check and bounce effect
      if (particle.x + particle.size > this.width) {
        particle.x = this.width - particle.size;
        particle.speedX *= -1;
      } else if (particle.x < 0) {
        particle.x = 0;
        particle.speedX *= -1;
      }
      
      if (particle.y + particle.size > this.height) {
        particle.y = this.height - particle.size;
        particle.speedY *= -1;
      } else if (particle.y < 0) {
        particle.y = 0;
        particle.speedY *= -1;
      }
      
      // Update DOM element position
      particle.element.style.left = `${particle.x}px`;
      particle.element.style.top = `${particle.y}px`;
    });
    
    // If particle connection feature is enabled, draw connection lines
    if (this.options.connectParticles) {
      this.connectParticles();
    }
    
    // If still animating, request next frame
    if (this.isAnimating) {
      this.rafId = requestAnimationFrame(this.update);
    }
  };

  private connectParticles(): void {
    // Remove existing connection lines
    const existingLines = this.container.querySelectorAll('.particle-line');
    existingLines.forEach(line => line.remove());
    
    // Iterate through all particle pairs, check the distance between them
    for (let i = 0; i < this.particles.length; i++) {
      for (let j = i + 1; j < this.particles.length; j++) {
        const p1 = this.particles[i];
        const p2 = this.particles[j];
        
        // Calculate distance between particle centers
        const dx = p1.x + p1.size / 2 - (p2.x + p2.size / 2);
        const dy = p1.y + p1.size / 2 - (p2.y + p2.size / 2);
        const distance = Math.sqrt(dx * dx + dy * dy);
        
        // If distance is less than set value, draw connection line
        if (distance < this.options.connectDistance) {
          // Calculate opacity, more distant = more transparent
          const opacity = 1 - distance / this.options.connectDistance;
          
          // Create line element
          const line = document.createElement('div');
          line.className = 'particle-line';
          line.style.position = 'absolute';
          line.style.width = `${distance}px`;
          line.style.height = '1px';
          line.style.backgroundColor = 'rgba(255, 255, 255, ' + opacity * 0.2 + ')';
          
          // Calculate line position and rotation
          const x1 = p1.x + p1.size / 2;
          const y1 = p1.y + p1.size / 2;
          const angle = Math.atan2(dy, dx);
          
          // Set line position and rotation
          line.style.left = `${x1}px`;
          line.style.top = `${y1}px`;
          line.style.transform = `rotate(${angle}rad)`;
          line.style.transformOrigin = '0 0';
          
          // Add to container
          this.container.appendChild(line);
        }
      }
    }
  }

  private handleResize = (): void => {
    // Update container dimensions
    this.width = this.container.offsetWidth;
    this.height = this.container.offsetHeight;
    
    // Recreate particles to adapt to new dimensions
    this.createParticles();
  };

  // Start animation
  public start(): void {
    if (!this.isAnimating) {
      this.isAnimating = true;
      this.rafId = requestAnimationFrame(this.update);
    }
  }

  // Stop animation
  public stop(): void {
    this.isAnimating = false;
    if (this.rafId !== null) {
      cancelAnimationFrame(this.rafId);
      this.rafId = null;
    }
  }

  // Destroy and release resources
  public destroy(): void {
    this.stop();
    this.removeAllParticles();
    
    if (this.resizeObserver) {
      this.resizeObserver.disconnect();
      this.resizeObserver = null;
    }
  }
}

export default ParticlesEffect; 