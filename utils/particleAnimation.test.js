import { initParticleAnimation } from './particleAnimation';

describe('Particle Animation Utility Function Tests', () => {
  let container;
  
  beforeEach(() => {
    // Create a DOM element as container
    container = document.createElement('div');
    document.body.appendChild(container);
  });
  
  afterEach(() => {
    // Clean up DOM elements
    if (container && container.parentNode) {
      container.parentNode.removeChild(container);
    }
    container = null;
  });
  
  test('Initialize particle animation and return cleanup function', () => {
    // Call particle animation initialization function
    const cleanup = initParticleAnimation(container, {
      particleCount: 10,
      color: 'rgba(255, 255, 255, 0.5)',
      connect: true
    });
    
    // Verify particles are added to the container
    expect(container.children.length).toBeGreaterThan(0);
    
    // Verify cleanup function is callable
    expect(typeof cleanup).toBe('function');
    
    // Call cleanup function
    cleanup();
    
    // Verify particles are cleaned up
    expect(container.querySelector('canvas')).toBeNull();
  });
  
  test('Initialize particles with default options', () => {
    // Call particle animation initialization function without options
    const cleanup = initParticleAnimation(container);
    
    // Verify particles are added to the container
    expect(container.children.length).toBeGreaterThan(0);
    
    // Call cleanup function
    cleanup();
  });
  
  test('Connection line functionality test', () => {
    // Call particle animation initialization function with connection enabled
    const cleanup = initParticleAnimation(container, {
      particleCount: 5,
      connect: true
    });
    
    // Verify canvas is created
    const canvas = container.querySelector('canvas');
    expect(canvas).not.toBeNull();
    
    // Call cleanup function
    cleanup();
  });
}); 